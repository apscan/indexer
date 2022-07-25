// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use crate::compression::{CompressedData, CompressionError};
use crate::{compression, Epoch, StorageServiceRequest};
use aptos_config::config::StorageServiceConfig;
use aptos_types::epoch_change::EpochChangeProof;
use aptos_types::ledger_info::LedgerInfoWithSignatures;
use aptos_types::state_store::state_value::StateValueChunkWithProof;
use aptos_types::transaction::{TransactionListWithProof, TransactionOutputListWithProof, Version};
use num_traits::{PrimInt, Zero};
#[cfg(test)]
use proptest::prelude::{any, Arbitrary, BoxedStrategy, Strategy};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt::{Debug, Display, Formatter};
use thiserror::Error;

#[derive(Clone, Debug, Deserialize, Error, PartialEq, Serialize)]
pub enum Error {
    #[error("Data range cannot be degenerate!")]
    DegenerateRangeError,
    #[error("Unexpected error encountered: {0}")]
    UnexpectedErrorEncountered(String),
}

impl From<CompressionError> for Error {
    fn from(error: CompressionError) -> Self {
        Error::UnexpectedErrorEncountered(error.to_string())
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
    pub fn new(storage_response: T, perform_compression: bool) -> Result<Self, Error> {
        let storage_response = if perform_compression {
            let raw_data = bcs::to_bytes(&storage_response)
                .map_err(|error| Error::UnexpectedErrorEncountered(error.to_string()))?;
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
    pub fn get_storage_response(&self) -> Result<T, Error> {
        let storage_response = match self {
            CompressibleStorageResponse::CompressedResponse(compressed_data) => {
                let raw_data = compression::decompress_data(compressed_data)?;
                bcs::from_bytes::<T>(&raw_data)
                    .map_err(|error| Error::UnexpectedErrorEncountered(error.to_string()))?
            }
            CompressibleStorageResponse::RawResponse(storage_response) => storage_response.clone(),
        };
        Ok(storage_response)
    }
}

impl TryFrom<StorageServiceResponse> for StateValueChunkWithProof {
    type Error = crate::responses::Error;
    fn try_from(response: StorageServiceResponse) -> Result<Self, Self::Error> {
        match response {
            StorageServiceResponse::StateValueChunkWithProof(response) => {
                response.get_storage_response()
            }
            _ => Err(Error::UnexpectedErrorEncountered(format!(
                "expected state_value_chunk_with_proof, found {}",
                response.get_label()
            ))),
        }
    }
}

impl TryFrom<StorageServiceResponse> for EpochChangeProof {
    type Error = crate::responses::Error;
    fn try_from(response: StorageServiceResponse) -> Result<Self, Self::Error> {
        match response {
            StorageServiceResponse::EpochEndingLedgerInfos(response) => {
                response.get_storage_response()
            }
            _ => Err(Error::UnexpectedErrorEncountered(format!(
                "expected epoch_ending_ledger_infos, found {}",
                response.get_label()
            ))),
        }
    }
}

impl TryFrom<StorageServiceResponse>
    for (TransactionOutputListWithProof, LedgerInfoWithSignatures)
{
    type Error = crate::responses::Error;
    fn try_from(response: StorageServiceResponse) -> Result<Self, Self::Error> {
        match response {
            StorageServiceResponse::NewTransactionOutputsWithProof(response) => {
                response.get_storage_response()
            }
            _ => Err(Error::UnexpectedErrorEncountered(format!(
                "expected new_transaction_outputs_with_proof, found {}",
                response.get_label()
            ))),
        }
    }
}

impl TryFrom<StorageServiceResponse> for (TransactionListWithProof, LedgerInfoWithSignatures) {
    type Error = crate::responses::Error;
    fn try_from(response: StorageServiceResponse) -> Result<Self, Self::Error> {
        match response {
            StorageServiceResponse::NewTransactionsWithProof(response) => {
                response.get_storage_response()
            }
            _ => Err(Error::UnexpectedErrorEncountered(format!(
                "expected new_transactions_with_proof, found {}",
                response.get_label()
            ))),
        }
    }
}

impl TryFrom<StorageServiceResponse> for u64 {
    type Error = crate::responses::Error;
    fn try_from(response: StorageServiceResponse) -> Result<Self, Self::Error> {
        match response {
            StorageServiceResponse::NumberOfStatesAtVersion(inner) => Ok(inner),
            _ => Err(Error::UnexpectedErrorEncountered(format!(
                "expected number_of_states_at_version, found {}",
                response.get_label()
            ))),
        }
    }
}

impl TryFrom<StorageServiceResponse> for ServerProtocolVersion {
    type Error = crate::responses::Error;
    fn try_from(response: StorageServiceResponse) -> Result<Self, Self::Error> {
        match response {
            StorageServiceResponse::ServerProtocolVersion(inner) => Ok(inner),
            _ => Err(Error::UnexpectedErrorEncountered(format!(
                "expected server_protocol_version, found {}",
                response.get_label()
            ))),
        }
    }
}

impl TryFrom<StorageServiceResponse> for StorageServerSummary {
    type Error = crate::responses::Error;
    fn try_from(response: StorageServiceResponse) -> Result<Self, Self::Error> {
        match response {
            StorageServiceResponse::StorageServerSummary(inner) => Ok(inner),
            _ => Err(Error::UnexpectedErrorEncountered(format!(
                "expected storage_server_summary, found {}",
                response.get_label()
            ))),
        }
    }
}

impl TryFrom<StorageServiceResponse> for TransactionOutputListWithProof {
    type Error = crate::responses::Error;
    fn try_from(response: StorageServiceResponse) -> Result<Self, Self::Error> {
        match response {
            StorageServiceResponse::TransactionOutputsWithProof(response) => {
                response.get_storage_response()
            }
            _ => Err(Error::UnexpectedErrorEncountered(format!(
                "expected transaction_outputs_with_proof, found {}",
                response.get_label()
            ))),
        }
    }
}

impl TryFrom<StorageServiceResponse> for TransactionListWithProof {
    type Error = crate::responses::Error;
    fn try_from(response: StorageServiceResponse) -> Result<Self, Self::Error> {
        match response {
            StorageServiceResponse::TransactionsWithProof(response) => {
                response.get_storage_response()
            }
            _ => Err(Error::UnexpectedErrorEncountered(format!(
                "expected transactions_with_proof, found {}",
                response.get_label()
            ))),
        }
    }
}

/// The protocol version run by this server. Clients request this first to
/// identify what API calls and data requests the server supports.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ServerProtocolVersion {
    pub protocol_version: u64, // The storage server version run by this instance.
}

/// A storage server summary, containing a summary of the information held
/// by the corresponding server instance. This is useful for identifying the
/// data that a server instance can provide, as well as relevant metadata.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct StorageServerSummary {
    pub protocol_metadata: ProtocolMetadata,
    pub data_summary: DataSummary,
}

impl StorageServerSummary {
    pub fn can_service(&self, request: &StorageServiceRequest) -> bool {
        self.protocol_metadata.can_service(request) && self.data_summary.can_service(request)
    }
}

/// A summary of the protocol metadata for the storage service instance, such as
/// the maximum chunk sizes supported for different requests.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProtocolMetadata {
    pub max_epoch_chunk_size: u64, // The max number of epochs the server can return in a single chunk
    pub max_state_chunk_size: u64, // The max number of states the server can return in a single chunk
    pub max_transaction_chunk_size: u64, // The max number of transactions the server can return in a single chunk
    pub max_transaction_output_chunk_size: u64, // The max number of transaction outputs the server can return in a single chunk
}

impl ProtocolMetadata {
    /// Returns true iff the request can be serviced
    pub fn can_service(&self, request: &StorageServiceRequest) -> bool {
        use crate::StorageServiceRequest::*;
        match request {
            GetNewTransactionsWithProof(_)
            | GetNewTransactionOutputsWithProof(_)
            | GetNumberOfStatesAtVersion(_)
            | GetServerProtocolVersion
            | GetStorageServerSummary => true,
            GetStateValuesWithProof(request) => CompleteDataRange::new(
                request.start_index,
                request.end_index,
            )
            .map_or(false, |range| {
                range
                    .len()
                    .map_or(false, |chunk_size| self.max_state_chunk_size >= chunk_size)
            }),
            GetEpochEndingLedgerInfos(request) => CompleteDataRange::new(
                request.start_epoch,
                request.expected_end_epoch,
            )
            .map_or(false, |range| {
                range
                    .len()
                    .map_or(false, |chunk_size| self.max_epoch_chunk_size >= chunk_size)
            }),
            GetTransactionOutputsWithProof(request) => CompleteDataRange::new(
                request.start_version,
                request.end_version,
            )
            .map_or(false, |range| {
                range.len().map_or(false, |chunk_size| {
                    self.max_transaction_output_chunk_size >= chunk_size
                })
            }),
            GetTransactionsWithProof(request) => CompleteDataRange::new(
                request.start_version,
                request.end_version,
            )
            .map_or(false, |range| {
                range.len().map_or(false, |chunk_size| {
                    self.max_transaction_chunk_size >= chunk_size
                })
            }),
        }
    }
}

impl Default for ProtocolMetadata {
    fn default() -> Self {
        let config = StorageServiceConfig::default();
        Self {
            max_epoch_chunk_size: config.max_epoch_chunk_size,
            max_transaction_chunk_size: config.max_transaction_chunk_size,
            max_transaction_output_chunk_size: config.max_transaction_output_chunk_size,
            max_state_chunk_size: config.max_state_chunk_size,
        }
    }
}

/// A summary of the data actually held by the storage service instance.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct DataSummary {
    /// The ledger info corresponding to the highest synced version in storage.
    /// This indicates the highest version and epoch that storage can prove.
    pub synced_ledger_info: Option<LedgerInfoWithSignatures>,
    /// The range of epoch ending ledger infos in storage, e.g., if the range
    /// is [(X,Y)], it means all epoch ending ledger infos for epochs X->Y
    /// (inclusive) are held.
    pub epoch_ending_ledger_infos: Option<CompleteDataRange<Epoch>>,
    /// The range of states held in storage, e.g., if the range is
    /// [(X,Y)], it means all states are held for every version X->Y
    /// (inclusive).
    pub states: Option<CompleteDataRange<Version>>,
    /// The range of transactions held in storage, e.g., if the range is
    /// [(X,Y)], it means all transactions for versions X->Y (inclusive) are held.
    pub transactions: Option<CompleteDataRange<Version>>,
    /// The range of transaction outputs held in storage, e.g., if the range
    /// is [(X,Y)], it means all transaction outputs for versions X->Y
    /// (inclusive) are held.
    pub transaction_outputs: Option<CompleteDataRange<Version>>,
}

impl DataSummary {
    /// Returns true iff the request can be serviced
    pub fn can_service(&self, request: &StorageServiceRequest) -> bool {
        use crate::StorageServiceRequest::*;
        match request {
            GetNewTransactionsWithProof(_)
            | GetNewTransactionOutputsWithProof(_)
            | GetServerProtocolVersion
            | GetStorageServerSummary => true,
            GetEpochEndingLedgerInfos(request) => {
                let desired_range =
                    match CompleteDataRange::new(request.start_epoch, request.expected_end_epoch) {
                        Ok(desired_range) => desired_range,
                        Err(_) => return false,
                    };
                self.epoch_ending_ledger_infos
                    .map(|range| range.superset_of(&desired_range))
                    .unwrap_or(false)
            }
            GetNumberOfStatesAtVersion(version) => self
                .states
                .map(|range| range.contains(*version))
                .unwrap_or(false),
            GetStateValuesWithProof(request) => {
                let proof_version = request.version;

                let can_serve_states = self
                    .states
                    .map(|range| range.contains(request.version))
                    .unwrap_or(false);

                let can_create_proof = self
                    .synced_ledger_info
                    .as_ref()
                    .map(|li| li.ledger_info().version() >= proof_version)
                    .unwrap_or(false);

                can_serve_states && can_create_proof
            }
            GetTransactionOutputsWithProof(request) => {
                let desired_range =
                    match CompleteDataRange::new(request.start_version, request.end_version) {
                        Ok(desired_range) => desired_range,
                        Err(_) => return false,
                    };

                let can_serve_outputs = self
                    .transaction_outputs
                    .map(|range| range.superset_of(&desired_range))
                    .unwrap_or(false);

                let can_create_proof = self
                    .synced_ledger_info
                    .as_ref()
                    .map(|li| li.ledger_info().version() >= request.proof_version)
                    .unwrap_or(false);

                can_serve_outputs && can_create_proof
            }
            GetTransactionsWithProof(request) => {
                let desired_range =
                    match CompleteDataRange::new(request.start_version, request.end_version) {
                        Ok(desired_range) => desired_range,
                        Err(_) => return false,
                    };

                let can_serve_txns = self
                    .transactions
                    .map(|range| range.superset_of(&desired_range))
                    .unwrap_or(false);

                let can_create_proof = self
                    .synced_ledger_info
                    .as_ref()
                    .map(|li| li.ledger_info().version() >= request.proof_version)
                    .unwrap_or(false);

                can_serve_txns && can_create_proof
            }
        }
    }
}

#[derive(Clone, Debug, Error)]
#[error("data range cannot be degenerate")]
pub struct DegenerateRangeError;

/// A struct representing a contiguous, non-empty data range (lowest to highest,
/// inclusive) where data is complete (i.e. there are no missing pieces of data).
///
/// This is used to provide a summary of the data currently held in storage, e.g.
/// a CompleteDataRange<Version> of (A,B) means all versions A->B (inclusive).
///
/// Note: `CompleteDataRanges` are never degenerate (lowest > highest) and the
/// range length is always expressible without overflowing. Constructing a
/// degenerate range via `new` will return an `Err`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CompleteDataRange<T> {
    lowest: T,
    highest: T,
}

fn range_length_checked<T: PrimInt>(
    lowest: T,
    highest: T,
) -> crate::Result<T, DegenerateRangeError> {
    // len = highest - lowest + 1
    // Note: the order of operations here is important; we need to subtract first
    // before we (+1) to ensure we don't underflow when highest == lowest.
    highest
        .checked_sub(&lowest)
        .and_then(|value| value.checked_add(&T::one()))
        .ok_or(DegenerateRangeError)
}

impl<T: PrimInt> CompleteDataRange<T> {
    pub fn new(lowest: T, highest: T) -> crate::Result<Self, DegenerateRangeError> {
        if lowest > highest || range_length_checked(lowest, highest).is_err() {
            Err(DegenerateRangeError)
        } else {
            Ok(Self { lowest, highest })
        }
    }

    /// Create a data range given the lower bound and the length of the range.
    pub fn from_len(lowest: T, len: T) -> crate::Result<Self, DegenerateRangeError> {
        // highest = lowest + len - 1
        // Note: the order of operations here is important
        let highest = len
            .checked_sub(&T::one())
            .and_then(|addend| lowest.checked_add(&addend))
            .ok_or(DegenerateRangeError)?;
        Self::new(lowest, highest)
    }

    #[inline]
    pub fn lowest(&self) -> T {
        self.lowest
    }

    #[inline]
    pub fn highest(&self) -> T {
        self.highest
    }

    /// Returns the length of the data range.
    #[inline]
    pub fn len(&self) -> crate::Result<T, DegenerateRangeError> {
        self.highest
            .checked_sub(&self.lowest)
            .and_then(|value| value.checked_add(&T::one()))
            .ok_or(DegenerateRangeError)
    }

    /// Returns true iff the given item is within this range
    pub fn contains(&self, item: T) -> bool {
        self.lowest <= item && item <= self.highest
    }

    /// Returns true iff this range is a superset of the other data range.
    pub fn superset_of(&self, other: &Self) -> bool {
        self.lowest <= other.lowest && other.highest <= self.highest
    }
}

impl<T: Zero> CompleteDataRange<T> {
    pub fn from_genesis(highest: T) -> Self {
        Self {
            lowest: T::zero(),
            highest,
        }
    }
}

impl<'de, T> serde::Deserialize<'de> for CompleteDataRange<T>
where
    T: PrimInt + serde::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> crate::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;

        #[derive(Deserialize)]
        #[serde(rename = "CompleteDataRange")]
        struct Value<U> {
            lowest: U,
            highest: U,
        }

        let value = Value::<T>::deserialize(deserializer)?;
        Self::new(value.lowest, value.highest).map_err(D::Error::custom)
    }
}

#[cfg(test)]
impl<T> Arbitrary for CompleteDataRange<T>
where
    T: PrimInt + Arbitrary + 'static,
{
    type Parameters = ();
    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        (any::<T>(), any::<T>())
            .prop_filter_map("degenerate range", |(lowest, highest)| {
                CompleteDataRange::new(lowest, highest).ok()
            })
            .boxed()
    }

    type Strategy = BoxedStrategy<Self>;
}
