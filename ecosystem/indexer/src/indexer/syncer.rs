// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0
use crate::{
    database::PgDbPool,
    indexer::{
        errors::TransactionProcessingError,
        fetcher::{TransactionFetcher, TransactionFetcherTrait},
        processing_result::ProcessingResult,
        transactions_processor::BatchTransactionsProcessor
    }
};
use anyhow::Result;
use aptos_rest_client::Transaction;
use std::sync::Arc;
use tokio::{sync::Mutex};
use url::{ParseError, Url};

use super::tailer::recurse_remove_null_bytes_from_json;

#[derive(Clone)]
pub struct Syncer {
    transaction_fetcher: Arc<Mutex<dyn TransactionFetcherTrait>>,
    processor: Arc<dyn BatchTransactionsProcessor>,
    connection_pool: PgDbPool,
}

pub fn remove_null_bytes_from_txn(txn: Transaction) -> Transaction {
    let mut txn_json = serde_json::to_value(txn).unwrap();
    recurse_remove_null_bytes_from_json(&mut txn_json);
    serde_json::from_value::<Transaction>(txn_json).unwrap()
}


impl Syncer {
    pub fn new(node_url: &str, connection_pool: PgDbPool, processor: Arc<dyn BatchTransactionsProcessor>) -> Result<Self, ParseError> {
        let url = Url::parse(node_url)?;
        let transaction_fetcher = TransactionFetcher::new(url, None);
        Ok(Self {
            transaction_fetcher: Arc::new(Mutex::new(transaction_fetcher)),
            processor: processor,
            connection_pool,
        })
    }

    pub async fn process_next_batch(
        &mut self,
        batch_size: u8,
    ) -> Result<ProcessingResult, TransactionProcessingError> {
        let txns = self.transaction_fetcher.lock().await.fetch_next_batch(batch_size).await;
        self.processor.process_transactions_with_status(txns).await
    }
}