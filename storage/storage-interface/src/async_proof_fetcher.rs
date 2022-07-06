// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use crate::{proof_fetcher::ProofFetcher, DbReader};

use aptos_crypto::{_once_cell::sync::Lazy, hash::CryptoHash, HashValue};
use aptos_metrics_core::{register_histogram_vec, HistogramVec};
use aptos_types::{
    proof::SparseMerkleProof,
    state_store::{state_key::StateKey, state_value::StateValue},
    transaction::Version,
};
use crossbeam_channel::{unbounded, Receiver, Sender};
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Instant,
};

pub struct AsyncProofFetcher {
    reader: Arc<dyn DbReader>,
    status_sender: Sender<DataCommand>,
    status_receiver: Receiver<DataCommand>,
    /// The worker thread handle, created upon Pruner instance construction and joined upon its
    /// destruction. It only becomes `None` after joined in `drop()`.
    num_proofs_to_read: AtomicUsize,
    pool: rayon::ThreadPool,
}

pub static FETCH_STATE_VALUE: Lazy<HistogramVec> = Lazy::new(|| {
    register_histogram_vec!(
        // metric name
        "fetch_state_value",
        // metric description
        "The total time spent in seconds of block execution in the block executor.",
        &["type"],
    )
    .unwrap()
});

impl AsyncProofFetcher {
    pub fn new(reader: Arc<dyn DbReader>) -> Self {
        let (data_sender, data_receiver) = unbounded();

        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(256)
            .build()
            .unwrap();

        Self {
            reader,
            status_sender: data_sender,
            status_receiver: data_receiver,
            num_proofs_to_read: AtomicUsize::new(0),
            pool,
        }
    }

    pub fn read_proof_async(&self, state_key: StateKey, version: Version) {
        let mut t = Instant::now();
        self.num_proofs_to_read.fetch_add(1, Ordering::Relaxed);
        FETCH_STATE_VALUE
            .with_label_values(&["atomic_fetch_add"])
            .observe(t.elapsed().as_secs_f64());
        t = Instant::now();
        let reader = self.reader.clone();
        let status_sender = self.status_sender.clone();
        self.pool.spawn(move || {
            Self::process_proof_read(reader, status_sender, state_key, version);
        });
        FETCH_STATE_VALUE
            .with_label_values(&["send"])
            .observe(t.elapsed().as_secs_f64());
    }

    pub fn finish_and_read_proofs(&self) -> HashMap<HashValue, SparseMerkleProof> {
        let mut proofs = HashMap::new();
        for _ in 0..self.num_proofs_to_read.load(Ordering::Relaxed) {
            let data = self
                .status_receiver
                .recv()
                .expect("Failed to receive proof on the channel");
            match data {
                DataCommand::Proof { proof } => {
                    proofs.insert(proof.0, proof.1);
                }
            }
        }
        // Reset the number of proofs to read for the next round.
        self.num_proofs_to_read.store(0, Ordering::Relaxed);
        proofs
    }

    fn process_proof_read(
        reader: Arc<dyn DbReader>,
        status_sender: Sender<DataCommand>,
        state_key: StateKey,
        version: Version,
    ) {
        let proof = reader
            .get_state_proof_by_version(&state_key, version)
            .expect("Proof reading should succeed");
        status_sender
            .send(DataCommand::Proof {
                proof: (state_key.hash(), proof),
            })
            .expect("Sending proof should succeed");
    }
}

impl ProofFetcher for AsyncProofFetcher {
    fn fetch_state_value_and_proof(
        &self,
        state_key: &StateKey,
        version: Version,
    ) -> anyhow::Result<(Option<StateValue>, Option<SparseMerkleProof>)> {
        // Send command to the async proof fetcher thread to read the proof in async.
        let mut t = Instant::now();
        self.read_proof_async(state_key.clone(), version);
        FETCH_STATE_VALUE
            .with_label_values(&["read_proof_async"])
            .observe(t.elapsed().as_secs_f64());
        t = Instant::now();
        let value = self.reader.get_state_value_by_version(state_key, version)?;
        FETCH_STATE_VALUE
            .with_label_values(&["get_state_value_by_version"])
            .observe(t.elapsed().as_secs_f64());
        Ok((value, None))
    }

    fn get_proof_cache(&self) -> HashMap<HashValue, SparseMerkleProof> {
        self.finish_and_read_proofs()
    }
}

pub enum DataCommand {
    /// Used to notify that all the threads finished reading the required proofs.
    Proof {
        proof: (HashValue, SparseMerkleProof),
    },
}
