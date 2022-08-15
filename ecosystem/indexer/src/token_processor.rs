// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use crate::models::token::{
    CreateCollectionEventType, CreateTokenDataEventType, MintTokenEventType,
    MutateTokenPropertyMapEventType, TokenData, TokenEvent,
};
use crate::schema::token_datas::dsl::token_datas;
use crate::schema::token_datas::{last_minted_at, supply};
use crate::{
    database::{execute_with_better_error, PgDbPool, PgPoolConnection},
    indexer::{
        errors::TransactionProcessingError, metadata_fetcher::MetaDataFetcher,
        processing_result::ProcessingResult, transaction_processor::TransactionProcessor,
    },
    models::{
        collection::Collection,
        events::EventModel,
        metadata::Metadata,
        ownership::Ownership,
        token_property::TokenProperty,
        transactions::{TransactionModel, UserTransaction},
    },
    schema,
    schema::ownerships::{dsl::amount as ownership_amount, ownership_id},
};
use aptos_rest_client::Transaction;
use async_trait::async_trait;
use diesel::{Connection, ExpressionMethods, QueryDsl, RunQueryDsl};
use futures::future::Either;
use std::{fmt::Debug, sync::Arc};

pub struct TokenTransactionProcessor {
    connection_pool: PgDbPool,
    index_token_uri: bool,
}

impl TokenTransactionProcessor {
    pub fn new(connection_pool: PgDbPool, index_token_uri: bool) -> Self {
        Self {
            connection_pool,
            index_token_uri,
        }
    }
}

impl Debug for TokenTransactionProcessor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state = &self.connection_pool.state();
        write!(
            f,
            "TokenTransactionProcessor {{ connections: {:?}  idle_connections: {:?} }}",
            state.connections, state.idle_connections
        )
    }
}

fn update_mint_token(
    conn: &PgPoolConnection,
    event_data: MintTokenEventType,
    txn: &UserTransaction,
) {
    let last_mint_time = txn.timestamp;

    // update the supply
    let result = diesel::update(token_datas.find(event_data.id.to_string()))
        .set((
            supply.eq(supply + event_data.amount.parse::<i64>().unwrap()),
            last_minted_at.eq(last_mint_time),
        ))
        .get_result::<TokenData>(conn);
    if result.is_err() {
        aptos_logger::warn!("Error running query: {:?}", result.as_ref().err().unwrap());
    }
}

async fn get_all_metadata(uris: &Vec<(String, String)>, res: &mut Vec<Metadata>) {
    let fetcher = MetaDataFetcher::new();
    for (tid, uri) in uris {
        let token_metadata = fetcher.get_metadata(uri.clone()).await;
        if token_metadata.is_some() {
            let metadata = Metadata::from_token_uri_meta(token_metadata.unwrap(), tid.clone());
            if metadata.is_some() {
                res.push(metadata.unwrap());
            }
        }
    }
}

fn insert_token_properties(
    conn: &PgPoolConnection,
    event_data: MutateTokenPropertyMapEventType,
    txn: &UserTransaction,
) {
    let token_property = TokenProperty {
        token_id: event_data.new_id.to_string(),
        previous_token_id: event_data.old_id.to_string(),
        property_keys: event_data.keys.to_string(),
        property_values: event_data.values.to_string(),
        property_types: event_data.types.to_string(),
        updated_at: txn.timestamp,
        inserted_at: chrono::Utc::now().naive_utc(),
    };
    execute_with_better_error(
        conn,
        diesel::insert_into(schema::token_propertys::table)
            .values(&token_property)
            .on_conflict_do_nothing(),
    )
    .expect("Error inserting row into token_properties");
}

fn insert_token_data(
    conn: &PgPoolConnection,
    event_data: CreateTokenDataEventType,
    txn: &UserTransaction,
) {
    let token_data = TokenData {
        token_data_id: event_data.id.to_string(),
        creator: event_data.id.creator,
        collection: event_data.id.collection,
        name: event_data.id.name,
        description: event_data.description,
        max_amount: event_data.maximum.to_string(),
        supply: 0, // supply only updated with mint event
        uri: event_data.uri,
        royalty_payee_address: event_data.royalty_payee_address,
        royalty_points_denominator: event_data.royalty_points_denominator.to_string(),
        royalty_points_numerator: event_data.royalty_points_numerator.to_string(),
        mutability_config: event_data.mutability_config.to_string(),
        property_keys: event_data.property_keys.to_string(),
        property_values: event_data.property_values.to_string(),
        property_types: event_data.property_types.to_string(),
        minted_at: txn.timestamp,
        inserted_at: chrono::Utc::now().naive_utc(),
        last_minted_at: txn.timestamp,
    };
    execute_with_better_error(
        conn,
        diesel::insert_into(schema::token_datas::table)
            .values(&token_data)
            .on_conflict_do_nothing(),
    )
    .expect("Error inserting row into token_datas");
}

fn update_token_ownership(
    conn: &PgPoolConnection,
    token_id: String,
    txn: &UserTransaction,
    amount_update: i64,
) {
    let ownership = Ownership::new(
        token_id,
        txn.sender.clone(),
        amount_update,
        txn.timestamp,
        chrono::Utc::now().naive_utc(),
    );
    execute_with_better_error(
        conn,
        diesel::insert_into(schema::ownerships::table)
            .values(&ownership)
            .on_conflict(ownership_id)
            .do_update()
            .set(ownership_amount.eq(ownership_amount + ownership.amount)),
    )
    .expect("Error update token ownership");
}

fn insert_collection(
    conn: &PgPoolConnection,
    event_data: CreateCollectionEventType,
    txn: &UserTransaction,
) {
    let collection = Collection::new(
        event_data.creator,
        event_data.collection_name,
        event_data.description,
        event_data.maximum,
        event_data.uri,
        txn.timestamp,
        chrono::Utc::now().naive_utc(),
    );
    execute_with_better_error(
        conn,
        diesel::insert_into(schema::collections::table)
            .values(&collection)
            .on_conflict_do_nothing(),
    )
    .expect("Error inserting row into collections");
}

fn process_token_on_chain_data(
    conn: &PgPoolConnection,
    events: &[EventModel],
    txn: &UserTransaction,
    uris: &mut Vec<(String, String)>,
) {
    // filter events to only keep token events
    let token_events = events
        .iter()
        .map(TokenEvent::from_event)
        .filter(|e| e.is_some())
        .collect::<Vec<Option<TokenEvent>>>();
    // for create token event, insert a new token to token table,
    // if token exists, increase the supply
    for event in token_events {
        match event.unwrap() {
            TokenEvent::CreateTokenDataEvent(event_data) => {
                let uri = event_data.uri.clone();
                let t_data_id = event_data.id.to_string();
                insert_token_data(conn, event_data, txn);
                uris.push((t_data_id, uri));
            }
            TokenEvent::MintTokenEvent(event_data) => {
                update_mint_token(conn, event_data, txn);
            }
            TokenEvent::CollectionCreationEvent(event_data) => {
                insert_collection(conn, event_data, txn);
            }
            TokenEvent::DepositEvent(event_data) => {
                update_token_ownership(conn, event_data.id.to_string(), txn, event_data.amount.parse::<i64>().unwrap());
            }
            TokenEvent::WithdrawEvent(event_data) => {
                update_token_ownership(conn, event_data.id.to_string(), txn, -event_data.amount.parse::<i64>().unwrap());
            }
            TokenEvent::MutateTokenPropertyMapEvent(event_data) => {
                insert_token_properties(conn, event_data, txn);
            }
            _ => (),
        }
    }
}

#[async_trait]
impl TransactionProcessor for TokenTransactionProcessor {
    fn name(&self) -> &'static str {
        "token_processor"
    }

    async fn process_transaction(
        &self,
        transaction: Arc<Transaction>,
    ) -> Result<ProcessingResult, TransactionProcessingError> {
        let version = transaction.version().unwrap_or(0);

        let (_, maybe_details_model, _, maybe_events, _) =
            TransactionModel::from_transaction(&transaction);

        let conn = self.get_conn();
        let mut token_uris: Vec<(String, String)> = vec![];

        let mut tx_result = conn.transaction::<(), diesel::result::Error, _>(|| {
            if let Some(Either::Left(user_txn)) = maybe_details_model {
                if let Some(events) = maybe_events {
                    process_token_on_chain_data(&conn, &events, &user_txn, &mut token_uris);
                }
            }
            Ok(())
        });

        if let Err(err) = tx_result {
            return Err(TransactionProcessingError::TransactionCommitError((
                anyhow::Error::from(err),
                version,
                self.name(),
            )));
        };
        if self.index_token_uri {
            let mut res: Vec<Metadata> = vec![];
            get_all_metadata(&token_uris, &mut res).await;
            tx_result = conn.transaction::<(), diesel::result::Error, _>(|| {
                for metadata in res {
                    execute_with_better_error(
                        &conn,
                        diesel::insert_into(schema::metadatas::table)
                            .values(&metadata)
                            .on_conflict_do_nothing(),
                    )
                    .expect("Error inserting row into metadatas");
                }
                Ok(())
            });
        }
        match tx_result {
            Ok(_) => Ok(ProcessingResult::new(self.name(), version)),
            Err(err) => Err(TransactionProcessingError::TransactionCommitError((
                anyhow::Error::from(err),
                version,
                self.name(),
            ))),
        }
    }

    fn connection_pool(&self) -> &PgDbPool {
        &self.connection_pool
    }
}
