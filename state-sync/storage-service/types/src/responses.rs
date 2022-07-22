// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use crate::compression::{CompressedData, CompressionError};
use crate::requests::ServerProtocolVersion;
use crate::{compression, StorageServerSummary};
use aptos_types::epoch_change::EpochChangeProof;
use aptos_types::ledger_info::LedgerInfoWithSignatures;
use aptos_types::state_store::state_value::StateValueChunkWithProof;
use aptos_types::transaction::{TransactionListWithProof, TransactionOutputListWithProof};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt::{Debug, Display, Formatter};
use thiserror::Error;

#[derive(Clone, Debug, Error)]
#[error("Unexpected error encountered: {0}")]
pub struct UnexpectedErrorEncountered(String);

impl From<CompressionError> for UnexpectedErrorEncountered {
    fn from(error: CompressionError) -> Self {
        UnexpectedErrorEncountered(error.to_string())
    }
}

/// A storage service response.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[allow(clippy::large_enum_variant)]
pub enum StorageServiceResponse {
    EpochEndingLedgerInfos(EpochEndingLedgerInfosResponse),
    NewTransactionOutputsWithProof(NewTransactionOutputsWithProofResponse),
    NewTransactionsWithProof(NewTransactionsWithProofResponse),
    NumberOfStatesAtVersion(u64),
    ServerProtocolVersion(ServerProtocolVersion),
    StateValueChunkWithProof(StateValueChunkWithProofResponse),
    StorageServerSummary(StorageServerSummary),
    TransactionOutputsWithProof(TransactionOutputsWithProofResponse),
    TransactionsWithProof(TransactionsWithProofResponse),
}

// TODO(philiphayes): is there a proc-macro for this?
impl StorageServiceResponse {
    /// Returns a summary label for the response
    pub fn get_label(&self) -> &'static str {
        match self {
            Self::EpochEndingLedgerInfos(response) => {
                if response.is_compressed() {
                    "compressed_epoch_ending_ledger_infos"
                } else {
                    "epoch_ending_ledger_infos"
                }
            }
            Self::NewTransactionOutputsWithProof(response) => {
                if response.is_compressed() {
                    "compressed_new_transaction_outputs_with_proof"
                } else {
                    "new_transaction_outputs_with_proof"
                }
            }
            Self::NewTransactionsWithProof(response) => {
                if response.is_compressed() {
                    "compressed_new_transactions_with_proof"
                } else {
                    "new_transactions_with_proof"
                }
            }
            Self::NumberOfStatesAtVersion(_) => "number_of_states_at_version",
            Self::ServerProtocolVersion(_) => "server_protocol_version",
            Self::StateValueChunkWithProof(response) => {
                if response.is_compressed() {
                    "compressed_state_value_chunk_with_proof"
                } else {
                    "state_value_chunk_with_proof"
                }
            }
            Self::StorageServerSummary(_) => "storage_server_summary",
            Self::TransactionOutputsWithProof(response) => {
                if response.is_compressed() {
                    "compressed_transaction_outputs_with_proof"
                } else {
                    "transaction_outputs_with_proof"
                }
            }
            Self::TransactionsWithProof(response) => {
                if response.is_compressed() {
                    "compressed_transactions_with_proof"
                } else {
                    "transactions_with_proof"
                }
            }
        }
    }
}

impl Display for StorageServiceResponse {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        // To prevent log spamming, we only display storage response data for summaries
        let data = match self {
            StorageServiceResponse::StorageServerSummary(storage_summary) => {
                format!("{:?}", storage_summary)
            }
            _ => "...".into(),
        };
        write!(
            f,
            "Storage service response: {}, data: {}",
            self.get_label(),
            data
        )
    }
}

/// A storage service response for fetching a list of epoch ending ledger infos.
pub type EpochEndingLedgerInfosResponse = CompressibleStorageResponse<EpochChangeProof>;

/// A storage service response for fetching a new transaction output list.
pub type NewTransactionOutputsWithProofResponse =
    CompressibleStorageResponse<(TransactionOutputListWithProof, LedgerInfoWithSignatures)>;

/// A storage service response for fetching a new transaction list.
pub type NewTransactionsWithProofResponse =
    CompressibleStorageResponse<(TransactionListWithProof, LedgerInfoWithSignatures)>;

/// A storage service response for fetching a list of state values.
pub type StateValueChunkWithProofResponse = CompressibleStorageResponse<StateValueChunkWithProof>;

/// A storage service response for fetching a transaction output list.
pub type TransactionOutputsWithProofResponse =
    CompressibleStorageResponse<TransactionOutputListWithProof>;

/// A storage service response for fetching a transaction list.
pub type TransactionsWithProofResponse = CompressibleStorageResponse<TransactionListWithProof>;

/// A storage service response that can be in a compressed format
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum CompressibleStorageResponse<
    T: Clone + Debug + DeserializeOwned + Eq + PartialEq + Serialize,
> {
    CompressedResponse(CompressedData),
    #[serde(bound = "")] // Workaround, see: https://github.com/serde-rs/serde/issues/1296
    RawResponse(T),
}

impl<T: Clone + Debug + DeserializeOwned + Eq + PartialEq + Serialize>
    CompressibleStorageResponse<T>
{
    /// Creates a new response and performs compression if required
    pub fn new(
        storage_response: T,
        perform_compression: bool,
    ) -> Result<Self, UnexpectedErrorEncountered> {
        let storage_response = if perform_compression {
            let raw_data = bcs::to_bytes(&storage_response)
                .map_err(|error| UnexpectedErrorEncountered(error.to_string()))?;
            let compressed_data = compression::compress_data(raw_data)?;
            CompressibleStorageResponse::CompressedResponse(compressed_data)
        } else {
            CompressibleStorageResponse::RawResponse(storage_response)
        };
        Ok(storage_response)
    }

    /// Returns true iff the response is compressed
    pub fn is_compressed(&self) -> bool {
        matches!(self, CompressibleStorageResponse::CompressedResponse(_))
    }

    /// Returns the storage response regardless of the inner format
    pub fn get_storage_response(&self) -> Result<T, UnexpectedErrorEncountered> {
        let storage_response = match self {
            CompressibleStorageResponse::CompressedResponse(compressed_data) => {
                let raw_data = compression::decompress_data(compressed_data)?;
                bcs::from_bytes::<T>(&raw_data)
                    .map_err(|error| UnexpectedErrorEncountered(error.to_string()))?
            }
            CompressibleStorageResponse::RawResponse(storage_response) => storage_response.clone(),
        };
        Ok(storage_response)
    }
}

// Conversions from the outer StorageServiceResponse enum to the inner types.
// TODO(philiphayes): is there a proc-macro for this?

impl TryFrom<StorageServiceResponse> for StateValueChunkWithProof {
    type Error = UnexpectedErrorEncountered;
    fn try_from(response: StorageServiceResponse) -> Result<Self, Self::Error> {
        match response {
            StorageServiceResponse::StateValueChunkWithProof(response) => {
                response.get_storage_response()
            }
            _ => Err(UnexpectedErrorEncountered(format!(
                "expected state_value_chunk_with_proof, found {}",
                response.get_label()
            ))),
        }
    }
}

impl TryFrom<StorageServiceResponse> for EpochChangeProof {
    type Error = UnexpectedErrorEncountered;
    fn try_from(response: StorageServiceResponse) -> Result<Self, Self::Error> {
        match response {
            StorageServiceResponse::EpochEndingLedgerInfos(response) => {
                response.get_storage_response()
            }
            _ => Err(UnexpectedErrorEncountered(format!(
                "expected epoch_ending_ledger_infos, found {}",
                response.get_label()
            ))),
        }
    }
}

impl TryFrom<StorageServiceResponse>
    for (TransactionOutputListWithProof, LedgerInfoWithSignatures)
{
    type Error = UnexpectedErrorEncountered;
    fn try_from(response: StorageServiceResponse) -> Result<Self, Self::Error> {
        match response {
            StorageServiceResponse::NewTransactionOutputsWithProof(response) => {
                response.get_storage_response()
            }
            _ => Err(UnexpectedErrorEncountered(format!(
                "expected new_transaction_outputs_with_proof, found {}",
                response.get_label()
            ))),
        }
    }
}

impl TryFrom<StorageServiceResponse> for (TransactionListWithProof, LedgerInfoWithSignatures) {
    type Error = UnexpectedErrorEncountered;
    fn try_from(response: StorageServiceResponse) -> Result<Self, Self::Error> {
        match response {
            StorageServiceResponse::NewTransactionsWithProof(response) => {
                response.get_storage_response()
            }
            _ => Err(UnexpectedErrorEncountered(format!(
                "expected new_transactions_with_proof, found {}",
                response.get_label()
            ))),
        }
    }
}

impl TryFrom<StorageServiceResponse> for u64 {
    type Error = UnexpectedErrorEncountered;
    fn try_from(response: StorageServiceResponse) -> Result<Self, Self::Error> {
        match response {
            StorageServiceResponse::NumberOfStatesAtVersion(inner) => Ok(inner),
            _ => Err(UnexpectedErrorEncountered(format!(
                "expected number_of_states_at_version, found {}",
                response.get_label()
            ))),
        }
    }
}

impl TryFrom<StorageServiceResponse> for ServerProtocolVersion {
    type Error = UnexpectedErrorEncountered;
    fn try_from(response: StorageServiceResponse) -> Result<Self, Self::Error> {
        match response {
            StorageServiceResponse::ServerProtocolVersion(inner) => Ok(inner),
            _ => Err(UnexpectedErrorEncountered(format!(
                "expected server_protocol_version, found {}",
                response.get_label()
            ))),
        }
    }
}

impl TryFrom<StorageServiceResponse> for StorageServerSummary {
    type Error = UnexpectedErrorEncountered;
    fn try_from(response: StorageServiceResponse) -> Result<Self, Self::Error> {
        match response {
            StorageServiceResponse::StorageServerSummary(inner) => Ok(inner),
            _ => Err(UnexpectedErrorEncountered(format!(
                "expected storage_server_summary, found {}",
                response.get_label()
            ))),
        }
    }
}

impl TryFrom<StorageServiceResponse> for TransactionOutputListWithProof {
    type Error = UnexpectedErrorEncountered;
    fn try_from(response: StorageServiceResponse) -> Result<Self, Self::Error> {
        match response {
            StorageServiceResponse::TransactionOutputsWithProof(response) => {
                response.get_storage_response()
            }
            _ => Err(UnexpectedErrorEncountered(format!(
                "expected transaction_outputs_with_proof, found {}",
                response.get_label()
            ))),
        }
    }
}

impl TryFrom<StorageServiceResponse> for TransactionListWithProof {
    type Error = UnexpectedErrorEncountered;
    fn try_from(response: StorageServiceResponse) -> Result<Self, Self::Error> {
        match response {
            StorageServiceResponse::TransactionsWithProof(response) => {
                response.get_storage_response()
            }
            _ => Err(UnexpectedErrorEncountered(format!(
                "expected transactions_with_proof, found {}",
                response.get_label()
            ))),
        }
    }
}
