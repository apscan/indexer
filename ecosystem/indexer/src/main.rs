// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

//! Indexer is used to index blockchain data into Postgres
//!
//! TODO: Examples
//!
#![forbid(unsafe_code)]

use aptos_logger::info;
use clap::Parser;
use std::sync::Arc;

use aptos_indexer::{
    database::new_db_pool, default_processor::DefaultTransactionProcessor, indexer::{tailer::Tailer, syncer::Syncer}, 
    batch_processor::BatchProcessor,
    token_processor::TokenTransactionProcessor,
};

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct IndexerArgs {
    /// Postgres database uri, ex: "postgresql://user:pass@localhost/postgres"
    #[clap(long)]
    pg_uri: String,

    /// URL of an Aptos node, ex: "https://fullnode.devnet.aptoslabs.com"
    #[clap(long)]
    node_url: String,

    /// If set, don't run any migrations
    #[clap(long)]
    skip_migrations: bool,

    /// If set, don't try to re-run all previous failed versions before tailing new ones
    #[clap(long)]
    skip_previous_errors: bool,

    /// If set, will exit after migrations/repairs instead of starting indexing loop
    #[clap(long)]
    dont_index: bool,

    /// If set, will ignore database contents and start processing from the specified version.
    /// This will not delete any database contents, just transactions as it reprocesses them.
    #[clap(long)]
    start_from_version: Option<u64>,

    /// How many versions to fetch and process from a node in parallel
    #[clap(long, default_value_t = 10)]
    batch_size: u8,

    /// How many versions to process before logging a "processed X versions" message.
    /// This will only be checked every `--batch-size` number of versions.
    /// Set to 0 to disable.
    #[clap(long, default_value_t = 1000)]
    emit_every: usize,

    /// Turn on the indexer to collect token, ownership, collection and metadata and store them
    /// in the postgres DB tables.
    #[clap(long)]
    index_token_data: bool,

    /// turn on the token URI fetcher
    #[clap(long)]
    index_token_uri_data: bool,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    aptos_logger::Logger::new().init();
    let args: IndexerArgs = IndexerArgs::parse();

    info!("Starting indexer...");

    let conn_pool = new_db_pool(&args.pg_uri).unwrap();
    info!("Created the connection pool... ");

    let mut tailer = Tailer::new(&args.node_url, conn_pool.clone()).unwrap();

    if !args.skip_migrations {
        tailer.run_migrations();
    }

    tailer.check_or_update_chain_id().await.unwrap();
    let pg_transaction_processor = DefaultTransactionProcessor::new(conn_pool.clone());    
    tailer.add_processor(Arc::new(pg_transaction_processor));
    if args.index_token_data {
        let token_transaction_processor =
            TokenTransactionProcessor::new(conn_pool.clone(), args.index_token_uri_data);
        tailer.add_processor(Arc::new(token_transaction_processor));
    }

    let _starting_version = match args.start_from_version {
        None => tailer.set_fetcher_to_lowest_processor_version().await,
        Some(version) => {
            print!("{}", version);
            tailer.set_fetcher_version(version).await}            
    };

    if !args.skip_previous_errors {
        tailer.handle_previous_errors().await;
    }

    if args.dont_index {
        info!("All pre-index tasks complete, exiting!");
        return Ok(());
    }

    info!("Indexing loop started!");

    let pg_batch_processor = BatchProcessor::new(conn_pool.clone());
    let mut syncer = Syncer::new(&args.node_url, conn_pool.clone()).unwrap();
    syncer.add_processor(Arc::new(pg_batch_processor));

    loop {
        let res = syncer.process_next_batch(args.batch_size).await;
        aptos_logger::info!("Indexer has processed versions {}", res.unwrap()[0].as_ref().unwrap().version);
    }
}
