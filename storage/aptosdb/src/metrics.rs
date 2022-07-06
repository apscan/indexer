// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use aptos_metrics_core::{
    exponential_buckets, register_histogram, register_histogram_vec, register_int_counter,
    register_int_counter_vec, register_int_gauge, register_int_gauge_vec, Histogram, HistogramVec,
    IntCounter, IntCounterVec, IntGauge, IntGaugeVec,
};
use once_cell::sync::Lazy;

pub static LEDGER_COUNTER: Lazy<IntGaugeVec> = Lazy::new(|| {
    register_int_gauge_vec!(
        // metric name
        "aptos_storage_ledger",
        // metric description
        "Aptos storage ledger counters",
        // metric labels (dimensions)
        &["type"]
    )
    .unwrap()
});

pub static COMMITTED_TXNS: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "aptos_storage_committed_txns",
        "Aptos storage committed transactions"
    )
    .unwrap()
});

pub static LATEST_TXN_VERSION: Lazy<IntGauge> = Lazy::new(|| {
    register_int_gauge!(
        "aptos_storage_latest_transaction_version",
        "Aptos storage latest transaction version"
    )
    .unwrap()
});

pub static LEDGER_VERSION: Lazy<IntGauge> = Lazy::new(|| {
    register_int_gauge!(
        "aptos_storage_ledger_version",
        "Version in the latest saved ledger info."
    )
    .unwrap()
});

pub static NEXT_BLOCK_EPOCH: Lazy<IntGauge> = Lazy::new(|| {
    register_int_gauge!(
        "aptos_storage_next_block_epoch",
        "ledger_info.next_block_epoch() for the latest saved ledger info."
    )
    .unwrap()
});

pub static STATE_ITEM_COUNT: Lazy<IntGauge> = Lazy::new(|| {
    register_int_gauge!(
        "aptos_storage_state_item_count",
        "Total number of entries in the StateDB at the latest version."
    )
    .unwrap()
});

pub static PRUNER_WINDOW: Lazy<IntGaugeVec> = Lazy::new(|| {
    register_int_gauge_vec!(
        // metric name
        "aptos_storage_prune_window",
        // metric description
        "Aptos storage prune window",
        // metric labels (dimensions)
        &["pruner_name",]
    )
    .unwrap()
});

/// DB pruner least readable versions
pub static PRUNER_LEAST_READABLE_VERSION: Lazy<IntGaugeVec> = Lazy::new(|| {
    register_int_gauge_vec!(
        // metric name
        "aptos_pruner_min_readable_version",
        // metric description
        "Aptos pruner least readable state version",
        // metric labels (dimensions)
        &["pruner_name",]
    )
    .unwrap()
});

pub static PRUNER_BATCH_SIZE: Lazy<IntGauge> =
    Lazy::new(|| register_int_gauge!("pruner_batch_size", "Aptos pruner batch size").unwrap());

pub static API_LATENCY_SECONDS: Lazy<HistogramVec> = Lazy::new(|| {
    register_histogram_vec!(
        // metric name
        "aptos_storage_api_latency_seconds",
        // metric description
        "Aptos storage api latency in seconds",
        // metric labels (dimensions)
        &["api_name", "result"],
        exponential_buckets(/*start=*/ 1e-6, /*factor=*/ 2.0, /*count=*/ 22).unwrap(),
    )
    .unwrap()
});

pub static OTHER_TIMERS_SECONDS: Lazy<HistogramVec> = Lazy::new(|| {
    register_histogram_vec!(
        // metric name
        "aptos_storage_other_timers_seconds",
        // metric description
        "Various timers below public API level.",
        // metric labels (dimensions)
        &["name"],
        exponential_buckets(/*start=*/ 1e-6, /*factor=*/ 2.0, /*count=*/ 22).unwrap(),
    )
    .unwrap()
});

/// Rocksdb metrics
pub static ROCKSDB_PROPERTIES: Lazy<IntGaugeVec> = Lazy::new(|| {
    register_int_gauge_vec!(
        // metric name
        "aptos_rocksdb_properties",
        // metric description
        "rocksdb integer properties",
        // metric labels (dimensions)
        &["cf_name", "property_name",]
    )
    .unwrap()
});

// Backup progress gauges:

pub(crate) static BACKUP_EPOCH_ENDING_EPOCH: Lazy<IntGauge> = Lazy::new(|| {
    register_int_gauge!(
        "aptos_backup_handler_epoch_ending_epoch",
        "Current epoch returned in an epoch ending backup."
    )
    .unwrap()
});

pub(crate) static BACKUP_TXN_VERSION: Lazy<IntGauge> = Lazy::new(|| {
    register_int_gauge!(
        "aptos_backup_handler_transaction_version",
        "Current version returned in a transaction backup."
    )
    .unwrap()
});

pub(crate) static BACKUP_STATE_SNAPSHOT_VERSION: Lazy<IntGauge> = Lazy::new(|| {
    register_int_gauge!(
        "aptos_backup_handler_state_snapshot_version",
        "Version of requested state snapshot backup."
    )
    .unwrap()
});

pub(crate) static BACKUP_STATE_SNAPSHOT_LEAF_IDX: Lazy<IntGauge> = Lazy::new(|| {
    register_int_gauge!(
        "aptos_backup_handler_state_snapshot_leaf_index",
        "Index of current leaf index returned in a state snapshot backup."
    )
    .unwrap()
});

pub static NODE_CACHE_HIT: Lazy<IntCounterVec> = Lazy::new(|| {
    register_int_counter_vec!(
        "aptos_storage_node_cache_hit",
        "Aptos storage state store node cache hit.",
        &["type"]
    )
    .unwrap()
});

pub static NODE_CACHE_TOTAL: Lazy<IntCounterVec> = Lazy::new(|| {
    register_int_counter_vec!(
        "aptos_storage_node_cache_total",
        "Aptos storage state store node cache total requests.",
        &["type"]
    )
    .unwrap()
});

pub static LRU_CACHE: Lazy<Histogram> = Lazy::new(|| {
    register_histogram!(
        "lru_cache_hit",
        "JMT lru cache hit latency.",
        exponential_buckets(/*start=*/ 1e-6, /*factor=*/ 2.0, /*count=*/ 22).unwrap(),
    )
    .unwrap()
});
pub static VERSION_CACHE: Lazy<Histogram> = Lazy::new(|| {
    register_histogram!(
        "version_cache_hit",
        "JMT version cache hit latency.",
        exponential_buckets(/*start=*/ 1e-6, /*factor=*/ 2.0, /*count=*/ 22).unwrap(),
    )
    .unwrap()
});
pub static CACHE_MISS_TOTAL: Lazy<Histogram> = Lazy::new(|| {
    register_histogram!(
        "cache_miss_total",
        "JMT lru cache miss total latency.",
        exponential_buckets(/*start=*/ 1e-6, /*factor=*/ 2.0, /*count=*/ 22).unwrap(),
    )
    .unwrap()
});

pub static CACHE_MISS_READ: Lazy<Histogram> = Lazy::new(|| {
    register_histogram!(
        "cache_miss_read",
        "JMT lru cache miss read latency.",
        exponential_buckets(/*start=*/ 1e-6, /*factor=*/ 2.0, /*count=*/ 22).unwrap(),
    )
    .unwrap()
});
pub static PROOF_READ: Lazy<Histogram> = Lazy::new(|| {
    register_histogram!(
        "proof_read",
        "proof read latency.",
        exponential_buckets(/*start=*/ 1e-6, /*factor=*/ 2.0, /*count=*/ 22).unwrap(),
    )
    .unwrap()
});

pub static VALUE_READ: Lazy<Histogram> = Lazy::new(|| {
    register_histogram!(
        "value_read",
        "value read latency.",
        exponential_buckets(/*start=*/ 1e-6, /*factor=*/ 2.0, /*count=*/ 22).unwrap(),
    )
    .unwrap()
});

pub static NODE_COUNT: Lazy<IntGauge> = Lazy::new(|| {
    register_int_gauge!(
        "aptos_storage_node_count",
        "Aptos storage node count per commit",
    )
    .unwrap()
});
