// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0
use crate::{
    database::{execute_with_better_error, PgDbPool},
    indexer::{
        errors::TransactionProcessingError,
        fetcher::{TransactionFetcher, TransactionFetcherTrait},
        processing_result::ProcessingResult,
        transactions_processor::BatchTransactionsProcessor
    }
};
use anyhow::Result;
use aptos_logger::info;
use aptos_rest_client::Transaction;
use std::{fmt::Debug, sync::Arc};
use tokio::{sync::Mutex, task::JoinHandle};
use url::{ParseError, Url};

use super::tailer::recurse_remove_null_bytes_from_json;

#[derive(Clone)]
pub struct Syncer {
    transaction_fetcher: Arc<Mutex<dyn TransactionFetcherTrait>>,
    processors: Vec<Arc<dyn BatchTransactionsProcessor>>,
    connection_pool: PgDbPool,
}

pub fn remove_null_bytes_from_txn(txn: Transaction) -> Transaction {
    let mut txn_json = serde_json::to_value(txn).unwrap();
    recurse_remove_null_bytes_from_json(&mut txn_json);
    serde_json::from_value::<Transaction>(txn_json).unwrap()
}


impl Syncer {
    pub fn new(node_url: &str, connection_pool: PgDbPool) -> Result<Self, ParseError> {
        let url = Url::parse(node_url)?;
        let transaction_fetcher = TransactionFetcher::new(url, None);
        Ok(Self {
            transaction_fetcher: Arc::new(Mutex::new(transaction_fetcher)),
            processors: vec![],
            connection_pool,
        })
    }

    pub fn add_processor(&mut self, processor: Arc<dyn BatchTransactionsProcessor>) {
        info!("Adding processor to indexer: {}", processor.name());
        self.processors.push(processor);
    }

    pub async fn process_next_batch(
        &mut self,
        batch_size: u8,
    ) -> anyhow::Result<Vec<Result<ProcessingResult, TransactionProcessingError>>> {
        let txns = self.fetch_next_batch(batch_size).await;
        self.process_transactions(txns).await
    }

    pub async fn process_transactions(
        &self,
        new_txns: Arc<Vec<Transaction>>,
    ) -> anyhow::Result<Vec<Result<ProcessingResult, TransactionProcessingError>>> {
        let mut tasks = vec![];
        for processor in &self.processors {
            let processor2 = processor.clone();
            let txns_2 = new_txns.clone();
            let task = tokio::task::spawn(async move {
                processor2
                    .process_transactions_with_status(txns_2.clone())
                    .await
            });
            tasks.push(task);
        }
        let results = await_tasks(tasks).await;
        Ok(results)
    }

    pub async fn fetch_next_batch(&mut self, batch_size : u8) -> Arc<Vec<Transaction>> {
        Arc::new(self.transaction_fetcher.lock().await.fetch_next_batch(batch_size).await)
    }
}

pub async fn await_tasks<T: Debug>(tasks: Vec<JoinHandle<T>>) -> Vec<T> {
    let mut results = vec![];
    for task in tasks {
        let result = task.await;
        if result.is_err() {
            aptos_logger::error!("Error joining task: {:?}", &result);
        }
        results.push(result.unwrap());
    }
    results
}