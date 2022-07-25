// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use crate::requests::{
    EpochEndingLedgerInfoRequest, StateValuesWithProofRequest, TransactionOutputsWithProofRequest,
    TransactionsWithProofRequest,
};
use crate::responses::{CompleteDataRange, DataSummary, ProtocolMetadata};
use crate::{compression, Epoch, StorageServiceRequest};
use aptos_crypto::ed25519::Ed25519PrivateKey;
use aptos_crypto::hash::HashValue;
use aptos_crypto::{PrivateKey, SigningKey, Uniform};
use aptos_types::account_address::AccountAddress;
use aptos_types::chain_id::ChainId;
use aptos_types::ledger_info::LedgerInfoWithSignatures;
use aptos_types::transaction::{
    ExecutionStatus, RawTransaction, Script, SignedTransaction, Transaction,
    TransactionListWithProof, TransactionOutput, TransactionOutputListWithProof,
    TransactionPayload, TransactionStatus, Version,
};
use aptos_types::write_set::WriteSet;
use aptos_types::{block_info::BlockInfo, ledger_info::LedgerInfo};
use claim::{assert_err, assert_ok};
use proptest::prelude::*;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::BTreeMap;
use std::fmt::Debug;

#[test]
fn test_compression() {
    // Test epoch ending ledger infos
    let epoch_ending_ledger_infos = create_epoch_ending_ledger_infos(0, 999);
    test_compress_and_decompress(epoch_ending_ledger_infos);

    // Test transaction outputs with proof
    let outputs_with_proof = create_output_list_with_proof(13434, 17000, 19000);
    test_compress_and_decompress(outputs_with_proof);

    // Test transactions with proof
    let transactions_with_proof = create_transaction_list_with_proof(1000, 1999, 1999, true);
    test_compress_and_decompress(transactions_with_proof);
}

#[test]
fn test_complete_data_range() {
    // good ranges
    assert_ok!(CompleteDataRange::new(0, 0));
    assert_ok!(CompleteDataRange::new(10, 10));
    assert_ok!(CompleteDataRange::new(10, 20));
    assert_ok!(CompleteDataRange::new(u64::MAX, u64::MAX));

    // degenerate ranges
    assert_err!(CompleteDataRange::new(1, 0));
    assert_err!(CompleteDataRange::new(20, 10));
    assert_err!(CompleteDataRange::new(u64::MAX, 0));
    assert_err!(CompleteDataRange::new(u64::MAX, 1));

    // range length overflow edge case
    assert_ok!(CompleteDataRange::new(1, u64::MAX));
    assert_ok!(CompleteDataRange::new(0, u64::MAX - 1));
    assert_err!(CompleteDataRange::new(0, u64::MAX));
}

#[test]
fn test_data_summary_can_service_epochs_request() {
    let summary = DataSummary {
        epoch_ending_ledger_infos: Some(create_range(100, 200)),
        ..Default::default()
    };

    // in range, can service

    assert!(summary.can_service(&create_get_epochs_request(100, 200)));
    assert!(summary.can_service(&create_get_epochs_request(125, 175)));
    assert!(summary.can_service(&create_get_epochs_request(100, 100)));
    assert!(summary.can_service(&create_get_epochs_request(150, 150)));
    assert!(summary.can_service(&create_get_epochs_request(200, 200)));

    // out of range, can't service

    assert!(!summary.can_service(&create_get_epochs_request(99, 200)));
    assert!(!summary.can_service(&create_get_epochs_request(100, 201)));
    assert!(!summary.can_service(&create_get_epochs_request(50, 250)));
    assert!(!summary.can_service(&create_get_epochs_request(50, 150)));
    assert!(!summary.can_service(&create_get_epochs_request(150, 250)));

    // degenerate range, can't service

    assert!(!summary.can_service(&create_get_epochs_request(150, 149)));
}

#[test]
fn test_data_summary_can_service_txns_request() {
    let summary = DataSummary {
        synced_ledger_info: Some(create_mock_ledger_info(250)),
        transactions: Some(create_range(100, 200)),
        ..Default::default()
    };

    // in range, can service

    assert!(summary.can_service(&create_get_txns_request(225, 100, 200)));
    assert!(summary.can_service(&create_get_txns_request(225, 125, 175)));
    assert!(summary.can_service(&create_get_txns_request(225, 100, 100)));
    assert!(summary.can_service(&create_get_txns_request(225, 150, 150)));
    assert!(summary.can_service(&create_get_txns_request(225, 200, 200)));
    assert!(summary.can_service(&create_get_txns_request(250, 200, 200)));

    // out of range, can't service

    assert!(!summary.can_service(&create_get_txns_request(225, 99, 200)));
    assert!(!summary.can_service(&create_get_txns_request(225, 100, 201)));
    assert!(!summary.can_service(&create_get_txns_request(225, 50, 250)));
    assert!(!summary.can_service(&create_get_txns_request(225, 50, 150)));
    assert!(!summary.can_service(&create_get_txns_request(225, 150, 250)));

    assert!(!summary.can_service(&create_get_txns_request(300, 100, 200)));
    assert!(!summary.can_service(&create_get_txns_request(300, 125, 175)));
    assert!(!summary.can_service(&create_get_txns_request(300, 100, 100)));
    assert!(!summary.can_service(&create_get_txns_request(300, 150, 150)));
    assert!(!summary.can_service(&create_get_txns_request(300, 200, 200)));
    assert!(!summary.can_service(&create_get_txns_request(251, 200, 200)));
}

#[test]
fn test_data_summary_can_service_txn_outputs_request() {
    let summary = DataSummary {
        synced_ledger_info: Some(create_mock_ledger_info(250)),
        transaction_outputs: Some(create_range(100, 200)),
        ..Default::default()
    };

    // in range and can provide proof => can service
    assert!(summary.can_service(&create_get_txn_outputs_request(225, 100, 200)));
    assert!(summary.can_service(&create_get_txn_outputs_request(225, 125, 175)));
    assert!(summary.can_service(&create_get_txn_outputs_request(225, 100, 100)));
    assert!(summary.can_service(&create_get_txn_outputs_request(225, 150, 150)));
    assert!(summary.can_service(&create_get_txn_outputs_request(225, 200, 200)));
    assert!(summary.can_service(&create_get_txn_outputs_request(250, 200, 200)));

    // can provide proof, but out of range => cannot service
    assert!(!summary.can_service(&create_get_txn_outputs_request(225, 99, 200)));
    assert!(!summary.can_service(&create_get_txn_outputs_request(225, 100, 201)));
    assert!(!summary.can_service(&create_get_txn_outputs_request(225, 50, 250)));
    assert!(!summary.can_service(&create_get_txn_outputs_request(225, 50, 150)));
    assert!(!summary.can_service(&create_get_txn_outputs_request(225, 150, 250)));

    // in range, but cannot provide proof => cannot service
    assert!(!summary.can_service(&create_get_txn_outputs_request(300, 100, 200)));
    assert!(!summary.can_service(&create_get_txn_outputs_request(300, 125, 175)));
    assert!(!summary.can_service(&create_get_txn_outputs_request(300, 100, 100)));
    assert!(!summary.can_service(&create_get_txn_outputs_request(300, 150, 150)));
    assert!(!summary.can_service(&create_get_txn_outputs_request(300, 200, 200)));
    assert!(!summary.can_service(&create_get_txn_outputs_request(251, 200, 200)));

    // invalid range
    assert!(!summary.can_service(&create_get_txn_outputs_request(225, 175, 125)));
}

#[test]
fn test_data_summary_can_service_state_chunk_request() {
    let summary = DataSummary {
        synced_ledger_info: Some(create_mock_ledger_info(250)),
        states: Some(create_range(100, 300)),
        ..Default::default()
    };

    // in range and can provide proof => can service
    assert!(summary.can_service(&create_get_states_request(100)));
    assert!(summary.can_service(&create_get_states_request(200)));
    assert!(summary.can_service(&create_get_states_request(250)));

    // in range, but cannot provide proof => cannot service
    assert!(!summary.can_service(&create_get_states_request(251)));
    assert!(!summary.can_service(&create_get_states_request(300)));

    // can provide proof, but out of range ==> cannot service
    assert!(!summary.can_service(&create_get_states_request(50)));
    assert!(!summary.can_service(&create_get_states_request(99)));
}

#[test]
fn test_protocol_metadata_can_service() {
    let metadata = ProtocolMetadata {
        max_transaction_chunk_size: 100,
        max_epoch_chunk_size: 100,
        max_transaction_output_chunk_size: 100,
        max_state_chunk_size: 100,
    };

    assert!(metadata.can_service(&create_get_txns_request(200, 100, 199)));
    assert!(!metadata.can_service(&create_get_txns_request(200, 100, 200)));

    assert!(metadata.can_service(&create_get_epochs_request(100, 199)));
    assert!(!metadata.can_service(&create_get_epochs_request(100, 200)));

    assert!(metadata.can_service(&create_get_txn_outputs_request(200, 100, 199)));
    assert!(!metadata.can_service(&create_get_txn_outputs_request(200, 100, 200)));

    assert!(metadata.can_service(&create_get_state_values_request(200, 100, 199)));
    assert!(!metadata.can_service(&create_get_state_values_request(200, 100, 200)));
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]

    #[test]
    fn test_data_summary_length_invariant(range in any::<CompleteDataRange<u64>>()) {
        // should not panic
        let _ = range.len();
    }
}

fn create_mock_ledger_info(version: Version) -> LedgerInfoWithSignatures {
    LedgerInfoWithSignatures::new(
        LedgerInfo::new(
            BlockInfo::new(0, 0, HashValue::zero(), HashValue::zero(), version, 0, None),
            HashValue::zero(),
        ),
        BTreeMap::new(),
    )
}

fn create_range(lowest: u64, highest: u64) -> CompleteDataRange<u64> {
    CompleteDataRange::new(lowest, highest).unwrap()
}

fn create_get_epochs_request(start: Epoch, end: Epoch) -> StorageServiceRequest {
    StorageServiceRequest::GetEpochEndingLedgerInfos(EpochEndingLedgerInfoRequest {
        start_epoch: start,
        expected_end_epoch: end,
        use_compression: true,
    })
}

fn create_get_txns_request(proof: Version, start: Version, end: Version) -> StorageServiceRequest {
    StorageServiceRequest::GetTransactionsWithProof(TransactionsWithProofRequest {
        proof_version: proof,
        start_version: start,
        end_version: end,
        include_events: true,
        use_compression: true,
    })
}

fn create_get_txn_outputs_request(
    proof_version: Version,
    start_version: Version,
    end_version: Version,
) -> StorageServiceRequest {
    StorageServiceRequest::GetTransactionOutputsWithProof(TransactionOutputsWithProofRequest {
        proof_version,
        start_version,
        end_version,
        use_compression: true,
    })
}

fn create_get_state_values_request(
    version: Version,
    start_index: u64,
    end_index: u64,
) -> StorageServiceRequest {
    StorageServiceRequest::GetStateValuesWithProof(StateValuesWithProofRequest {
        version,
        start_index,
        end_index,
        use_compression: true,
    })
}

fn create_get_states_request(version: Version) -> StorageServiceRequest {
    create_get_state_values_request(version, 0, 1000)
}

/// Ensures that the given object can be compressed and decompressed successfully
/// when BCS encoded.
fn test_compress_and_decompress<T: Debug + DeserializeOwned + PartialEq + Serialize>(object: T) {
    let bcs_encoded_bytes = bcs::to_bytes(&object).unwrap();
    let compressed_bytes = compression::compress_data(bcs_encoded_bytes).unwrap();
    let decompressed_bytes = compression::decompress_data(&compressed_bytes).unwrap();
    let decoded_object = bcs::from_bytes::<T>(&decompressed_bytes).unwrap();

    assert_eq!(object, decoded_object);
}

/// Creates a test epoch change proof
fn create_epoch_ending_ledger_infos(
    start_epoch: Epoch,
    end_epoch: Epoch,
) -> Vec<LedgerInfoWithSignatures> {
    let mut ledger_info_with_sigs = vec![];
    for epoch in start_epoch..end_epoch {
        ledger_info_with_sigs.push(create_test_ledger_info_with_sigs(epoch, 0));
    }
    ledger_info_with_sigs
}

/// Creates a test transaction output list with proof
fn create_output_list_with_proof(
    start_version: u64,
    end_version: u64,
    proof_version: u64,
) -> TransactionOutputListWithProof {
    let transaction_list_with_proof =
        create_transaction_list_with_proof(start_version, end_version, proof_version, false);
    let transactions_and_outputs = transaction_list_with_proof
        .transactions
        .iter()
        .map(|txn| (txn.clone(), create_test_transaction_output()))
        .collect();

    TransactionOutputListWithProof::new(
        transactions_and_outputs,
        Some(start_version),
        transaction_list_with_proof.proof,
    )
}

/// Creates a test ledger info with signatures
fn create_test_ledger_info_with_sigs(epoch: u64, version: u64) -> LedgerInfoWithSignatures {
    // Create a mock ledger info with signatures
    let ledger_info = LedgerInfo::new(
        BlockInfo::new(
            epoch,
            0,
            HashValue::zero(),
            HashValue::zero(),
            version,
            0,
            None,
        ),
        HashValue::zero(),
    );
    LedgerInfoWithSignatures::new(ledger_info, BTreeMap::new())
}

/// Creates a test transaction output
fn create_test_transaction_output() -> TransactionOutput {
    TransactionOutput::new(
        WriteSet::default(),
        vec![],
        0,
        TransactionStatus::Keep(ExecutionStatus::MiscellaneousError(None)),
    )
}

/// Creates a test user transaction
fn create_test_transaction(sequence_number: u64) -> Transaction {
    let private_key = Ed25519PrivateKey::generate_for_testing();
    let public_key = private_key.public_key();

    let transaction_payload = TransactionPayload::Script(Script::new(vec![], vec![], vec![]));
    let raw_transaction = RawTransaction::new(
        AccountAddress::random(),
        sequence_number,
        transaction_payload,
        0,
        0,
        0,
        ChainId::new(10),
    );
    let signed_transaction = SignedTransaction::new(
        raw_transaction.clone(),
        public_key,
        private_key.sign(&raw_transaction),
    );

    Transaction::UserTransaction(signed_transaction)
}

/// Creates a test transaction output list with proof
fn create_transaction_list_with_proof(
    start_version: u64,
    end_version: u64,
    _proof_version: u64,
    include_events: bool,
) -> TransactionListWithProof {
    // Include events if required
    let events = if include_events { Some(vec![]) } else { None };

    // Create the requested transactions
    let mut transactions = vec![];
    for sequence_number in start_version..=end_version {
        transactions.push(create_test_transaction(sequence_number));
    }

    // Create a transaction list with an empty proof
    let mut transaction_list_with_proof = TransactionListWithProof::new_empty();
    transaction_list_with_proof.first_transaction_version = Some(start_version);
    transaction_list_with_proof.events = events;
    transaction_list_with_proof.transactions = transactions;

    transaction_list_with_proof
}
