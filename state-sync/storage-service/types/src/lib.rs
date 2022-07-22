// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]

use crate::requests::StorageServiceRequest;
use crate::responses::StorageServiceResponse;
use aptos_config::config::StorageServiceConfig;
use aptos_types::{ledger_info::LedgerInfoWithSignatures, transaction::Version};
use num_traits::{int::PrimInt, Zero};
#[cfg(test)]
use proptest::{
    arbitrary::{any, Arbitrary},
    strategy::{BoxedStrategy, Strategy},
};
use serde::{de, Deserialize, Serialize};
use thiserror::Error;

pub mod compression;
mod metrics;
pub mod requests;
pub mod responses;

#[cfg(test)]
mod tests;

/// A type alias for different epochs.
pub type Epoch = u64;

pub type Result<T, E = StorageServiceError> = ::std::result::Result<T, E>;

/// A storage service error that can be returned to the client on a failure
/// to process a service request.
#[derive(Clone, Debug, Deserialize, Eq, Error, PartialEq, Serialize)]
pub enum StorageServiceError {
    #[error("Internal service error: {0}")]
    InternalError(String),
    #[error("Invalid storage request: {0}")]
    InvalidRequest(String),
}

/// A single storage service message sent or received over AptosNet.
#[derive(Clone, Debug, Deserialize, Serialize)]
// TODO(philiphayes): do something about this without making it ugly :(
#[allow(clippy::large_enum_variant)]
pub enum StorageServiceMessage {
    /// A request to the storage service.
    Request(StorageServiceRequest),
    /// A response from the storage service. If there was an error while handling
    /// the request, the service will return an [`StorageServiceError`] error.
    Response(Result<StorageServiceResponse>),
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
        use StorageServiceRequest::*;
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
        use StorageServiceRequest::*;
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

fn range_length_checked<T: PrimInt>(lowest: T, highest: T) -> Result<T, DegenerateRangeError> {
    // len = highest - lowest + 1
    // Note: the order of operations here is important; we need to subtract first
    // before we (+1) to ensure we don't underflow when highest == lowest.
    highest
        .checked_sub(&lowest)
        .and_then(|value| value.checked_add(&T::one()))
        .ok_or(DegenerateRangeError)
}

impl<T: PrimInt> CompleteDataRange<T> {
    pub fn new(lowest: T, highest: T) -> Result<Self, DegenerateRangeError> {
        if lowest > highest || range_length_checked(lowest, highest).is_err() {
            Err(DegenerateRangeError)
        } else {
            Ok(Self { lowest, highest })
        }
    }

    /// Create a data range given the lower bound and the length of the range.
    pub fn from_len(lowest: T, len: T) -> Result<Self, DegenerateRangeError> {
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
    pub fn len(&self) -> Result<T, DegenerateRangeError> {
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

impl<'de, T> de::Deserialize<'de> for CompleteDataRange<T>
where
    T: PrimInt + de::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
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
