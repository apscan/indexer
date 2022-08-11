// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use crate::{
    database::{execute_with_better_error, PgDbPool, PgPoolConnection},
    indexer::{
        errors::TransactionProcessingError, processing_result::ProcessingResult,
        transactions_processor::BatchTransactionsProcessor,
    },
    models::{
        events::EventModel,
        transactions::{BlockMetadataTransactionModel, TransactionModel, UserTransactionModel},
        write_set_changes::WriteSetChangeModel
    },
    schema,
};
use aptos_rest_client::Transaction;
use async_trait::async_trait;
use diesel::{Connection};
use std::{fmt::Debug, sync::Arc};

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

fn insert_events(conn: &PgPoolConnection, events: &Vec<EventModel>) {
    execute_with_better_error(
        conn,
        diesel::insert_into(schema::events::table)
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
        transactions: Arc<Vec<Transaction>>,
    ) -> Result<ProcessingResult, TransactionProcessingError> {
        let (transaction_models, user_transaction_models, block_metadata_transaction_models
            , events, write_set_changes) =
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

            if !events.is_empty() {
                insert_events(&conn, &events);
            };
            if !write_set_changes.is_empty() {
                insert_write_set_changes(&conn, &write_set_changes);
            };
            Ok(())
        });

        match tx_result {
            Ok(_) => Ok(ProcessingResult::new(self.name(), start_version)),
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
