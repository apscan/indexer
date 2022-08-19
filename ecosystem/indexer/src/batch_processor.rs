// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use crate::{
    database::{execute_with_better_error, PgDbPool, PgPoolConnection},
    indexer::{
        errors::TransactionProcessingError, processing_result::ProcessingResult,
        transactions_processor::BatchTransactionsProcessor,
    },
    models::{
        events::EventModelPlural,
        blocks::Block,
        transactions::{BlockMetadataTransactionModel, TransactionModel, UserTransactionModel},
        write_set_changes::{WriteSetChangeModel, WriteSetChangePlural}, payloads::TransactionPayloadPlural
    },
    schema,
};
use aptos_rest_client::Transaction;
use async_trait::async_trait;
use diesel::{Connection};
use std::fmt::Debug;

pub struct BatchProcessor {
    connection_pool: PgDbPool,
}

impl BatchProcessor {
    pub fn new(connection_pool: PgDbPool) -> Self {
        Self { connection_pool }
    }
}

impl Debug for BatchProcessor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state = &self.connection_pool.state();
        write!(
            f,
            "BatchProcessor {{ connections: {:?}  idle_connections: {:?} }}",
            state.connections, state.idle_connections
        )
    }
}

fn insert_event_plural(conn: &PgPoolConnection, event_plural: &EventModelPlural) {
    if !event_plural.events.is_empty() {
        execute_with_better_error(
            conn,
            diesel::insert_into(schema::events::table)
                .values(&event_plural.events)
                .on_conflict_do_nothing(),
        )
        .expect("Error inserting row into database");

        execute_with_better_error(
            conn,
            diesel::insert_into(schema::event_keys::table)
                .values(&event_plural.event_keys)
                .on_conflict_do_nothing(),
        )
        .expect("Error inserting row into database");        
    }
}

fn insert_block_events(conn: &PgPoolConnection, events: &Vec<Block>) {
    execute_with_better_error(
        conn,
        diesel::insert_into(schema::blocks::table)
            .values(events)
            .on_conflict_do_nothing(),
    )
    .expect("Error inserting row into database");
}

fn insert_write_set_changes(conn: &PgPoolConnection, write_set_changes: &Vec<WriteSetChangeModel>) {
    execute_with_better_error(
        conn,
        diesel::insert_into(schema::write_set_changes::table)
            .values(write_set_changes)
            .on_conflict_do_nothing(),
    )
    .expect("Error inserting row into database");
    execute_with_better_error(
        conn,
        diesel::insert_into(schema::write_set_changes::table)
            .values(write_set_changes)
            .on_conflict_do_nothing(),
    )
    .expect("Error inserting row into database");
}

fn insert_write_set_plural(conn: &PgPoolConnection, write_set_plural: &WriteSetChangePlural) {
    if !write_set_plural.module_changes.is_empty() {
        execute_with_better_error(
            conn,
            diesel::insert_into(schema::module_changes::table)
                .values(&write_set_plural.module_changes)
                .on_conflict_do_nothing(),
        )
        .expect("Error inserting row into database");
    }

    if !write_set_plural.resource_changes.is_empty() {
        execute_with_better_error(
            conn,
            diesel::insert_into(schema::resource_changes::table)
                .values(&write_set_plural.resource_changes)
                .on_conflict_do_nothing(),
        )
        .expect("Error inserting row into database");
    }

    if !write_set_plural.table_item_changes.is_empty() {
        execute_with_better_error(
            conn,
            diesel::insert_into(schema::table_item_changes::table)
                .values(&write_set_plural.table_item_changes)
                .on_conflict_do_nothing(),
        )
        .expect("Error inserting row into database");
    }
}

fn insert_payload_plural(conn: &PgPoolConnection, payload_plural: &TransactionPayloadPlural) {
    if !payload_plural.script_write_set_payloads.is_empty() {
        execute_with_better_error(
            conn,
            diesel::insert_into(schema::script_write_set_payloads::table)
                .values(&payload_plural.script_write_set_payloads)
                .on_conflict_do_nothing(),
        )
        .expect("Error inserting row into database");
    }

    if !payload_plural.direct_write_set_payloads.is_empty() {
        execute_with_better_error(
            conn,
            diesel::insert_into(schema::direct_write_set_payloads::table)
                .values(&payload_plural.direct_write_set_payloads)
                .on_conflict_do_nothing(),
        )
        .expect("Error inserting row into database");
    }

    if !payload_plural.script_function_payloads.is_empty() {
        execute_with_better_error(
            conn,
            diesel::insert_into(schema::script_function_payloads::table)
                .values(&payload_plural.script_function_payloads)
                .on_conflict_do_nothing(),
        )
        .expect("Error inserting row into database");
    }

    if !payload_plural.module_bundle_payloads.is_empty() {
        execute_with_better_error(
            conn,
            diesel::insert_into(schema::module_bundle_payloads::table)
                .values(&payload_plural.module_bundle_payloads)
                .on_conflict_do_nothing(),
        )
        .expect("Error inserting row into database");
    }

    if !payload_plural.script_payloads.is_empty() {
        execute_with_better_error(
            conn,
            diesel::insert_into(schema::script_payloads::table)
                .values(&payload_plural.script_payloads)
                .on_conflict_do_nothing(),
        )
        .expect("Error inserting row into database");
    }

}

fn insert_transactions(conn: &PgPoolConnection, start_version: u64, end_version : u64, transaction_models: &Vec<TransactionModel>) {
    aptos_logger::trace!(
        "[default_processor] inserting 'transactions' start_version {} end_version {}",
        start_version,
        end_version
    );
    execute_with_better_error(
        conn,
        diesel::insert_into(schema::transactions::table)
            .values(transaction_models)
            .on_conflict_do_nothing()
    )
            .expect("Error inserting rows into database");
}

fn insert_user_transactions(
    conn: &PgPoolConnection,
    start_version: u64, 
    end_version : u64,
    user_transaction_models: &Vec<UserTransactionModel>,
) {
    aptos_logger::trace!(
        "[default_processor] inserting 'user_transaction' start_version {} end_version {}",
        start_version,
        end_version
    );
    execute_with_better_error(
        conn,
        diesel::insert_into(schema::user_transactions::table)
            .values(user_transaction_models)
            .on_conflict_do_nothing()
    ).expect("Error inserting rows into database");
}

fn insert_block_metadata_transactions(
    conn: &PgPoolConnection,
    start_version: u64, 
    end_version : u64,
    block_metadata_transaction_models: &Vec<BlockMetadataTransactionModel>,
) {
    aptos_logger::trace!(
        "[default_processor] inserting 'block_metadata_transaction' start_version {} end_version {}",
        start_version,
        end_version    
    );
    execute_with_better_error(
        conn,
        diesel::insert_into(schema::block_metadata_transactions::table)
            .values(block_metadata_transaction_models)
            .on_conflict_do_nothing()
    )
    .expect("Error inserting row into database");
}

#[async_trait]
impl BatchTransactionsProcessor for BatchProcessor {
    fn name(&self) -> &'static str {
        "batch_processor"
    }

    async fn process_transactions(
        &self,
        transactions: Vec<Transaction>,
    ) -> Result<ProcessingResult, TransactionProcessingError> {
        let (transaction_models, user_transaction_models, block_metadata_transaction_models
            , payload_plural, event_plural, block_events, write_set_changes, write_set_plural) =
            TransactionModel::from_transactions(&transactions);

        let start_version = transactions[0].version().unwrap_or(0);
        let end_version = transactions.last().unwrap().version().unwrap_or(0);
        let conn = self.get_conn();

        let tx_result = conn.transaction::<(), diesel::result::Error, _>(|| {
            insert_transactions(&conn, start_version, end_version, &transaction_models);

            if !user_transaction_models.is_empty() {
                insert_user_transactions(&conn, start_version, end_version, &user_transaction_models);
            }

            if !block_metadata_transaction_models.is_empty() {
                insert_block_metadata_transactions(&conn, start_version, end_version, &block_metadata_transaction_models);
            }

            insert_payload_plural(&conn, &payload_plural);
            
            insert_event_plural(&conn, &event_plural);

            if !block_events.is_empty() {
                insert_block_events(&conn, &block_events);
            };

            if !write_set_changes.is_empty() {
                insert_write_set_changes(&conn, &write_set_changes);
            };

            insert_write_set_plural(&conn, &write_set_plural);

            Ok(())
        });

        match tx_result {
            Ok(_) => Ok(ProcessingResult::new(self.name(), end_version)),
            Err(err) => Err(TransactionProcessingError::TransactionCommitError((
                anyhow::Error::from(err),
                start_version,
                self.name(),
            ))),
        }
    }

    fn connection_pool(&self) -> &PgDbPool {
        &self.connection_pool
    }
}
