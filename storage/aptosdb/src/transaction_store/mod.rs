// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

//! This file defines transaction store APIs that are related to committed signed transactions.

use crate::{
    change_set::ChangeSet,
    errors::AptosDbError,
    schema::{
        transaction::TransactionSchema, transaction_by_account::TransactionByAccountSchema,
        transaction_by_hash::TransactionByHashSchema, write_set::WriteSetSchema,
    },
    transaction_info::TransactionInfoSchema,
};
use anyhow::{ensure, format_err, Result};
use aptos_crypto::{hash::CryptoHash, HashValue};
use aptos_types::{
    account_address::AccountAddress,
    proof::position::Position,
    transaction::{Transaction, Version},
    write_set::WriteSet,
};
use schemadb::{ReadOptions, SchemaBatch, SchemaIterator, DB};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct TransactionStore {
    db: Arc<DB>,
}

impl TransactionStore {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    /// Gets the version of a transaction by the sender `address` and `sequence_number`.
    pub fn get_account_transaction_version(
        &self,
        address: AccountAddress,
        sequence_number: u64,
        ledger_version: Version,
    ) -> Result<Option<Version>> {
        if let Some(version) = self
            .db
            .get::<TransactionByAccountSchema>(&(address, sequence_number))?
        {
            if version <= ledger_version {
                return Ok(Some(version));
            }
        }

        Ok(None)
    }

    /// Gets the version of a transaction by its hash.
    pub fn get_transaction_version_by_hash(
        &self,
        hash: &HashValue,
        ledger_version: Version,
    ) -> Result<Option<Version>> {
        Ok(match self.db.get::<TransactionByHashSchema>(hash)? {
            Some(version) if version <= ledger_version => Some(version),
            _ => None,
        })
    }

    /// Gets an iterator that yields `(sequence_number, version)` for each
    /// transaction sent by an account, with minimum sequence number greater
    /// `min_seq_num`, and returning at most `num_versions` results with
    /// `version <= ledger_version`.
    /// Guarantees that the returned sequence numbers are sequential, i.e.,
    /// `seq_num_{i} + 1 = seq_num_{i+1}`.
    pub fn get_account_transaction_version_iter(
        &self,
        address: AccountAddress,
        min_seq_num: u64,
        num_versions: u64,
        ledger_version: Version,
    ) -> Result<AccountTransactionVersionIter> {
        let mut iter = self
            .db
            .iter::<TransactionByAccountSchema>(ReadOptions::default())?;
        iter.seek(&(address, min_seq_num))?;
        Ok(AccountTransactionVersionIter {
            inner: iter,
            address,
            expected_next_seq_num: None,
            end_seq_num: min_seq_num
                .checked_add(num_versions)
                .ok_or_else(|| format_err!("too many transactions requested"))?,
            prev_version: None,
            ledger_version,
        })
    }

    /// Get signed transaction given `version`
    pub fn get_transaction(&self, version: Version) -> Result<Transaction> {
        self.db
            .get::<TransactionSchema>(&version)?
            .ok_or_else(|| AptosDbError::NotFound(format!("Txn {}", version)).into())
    }

    /// Gets an iterator that yields `num_transactions` transactions starting from `start_version`.
    pub fn get_transaction_iter(
        &self,
        start_version: Version,
        num_transactions: usize,
    ) -> Result<TransactionIter> {
        let mut iter = self.db.iter::<TransactionSchema>(ReadOptions::default())?;
        iter.seek(&start_version)?;
        Ok(TransactionIter {
            inner: iter,
            expected_next_version: start_version,
            end_version: start_version
                .checked_add(num_transactions as u64)
                .ok_or_else(|| format_err!("too many transactions requested"))?,
        })
    }

    /// Get the first version that txn starts existent.
    pub fn get_first_txn_version(&self) -> Result<Option<Version>> {
        let mut iter = self.db.iter::<TransactionSchema>(Default::default())?;
        iter.seek_to_first();
        iter.next().map(|res| res.map(|(v, _)| v)).transpose()
    }

    /// Searches around the version to find the block's transactions
    pub fn get_block_boundaries(
        &self,
        version: Version,
        latest_ledger_version: Version,
    ) -> Result<(Version, Version)> {
        // Must be larger than a block size, otherwise a NotFound error will be raised wrongly.
        const MAX_VERSIONS_TO_SEARCH: usize = 1000 * 100;

        // Genesis is always 0,0
        if version == 0 {
            return Ok((0, 0));
        }

        // Linear searches via `DB::iter()` and `DB::rev_iter()` here, NOT expecting performance hit, due to the fact
        // that the iterator caches data block and that there are limited number of transactions in
        // each block.
        let mut iter = self.db.rev_iter::<TransactionSchema>(Default::default())?;
        iter.seek(&version)?;
        let mut start_version = None;
        for res in iter.take(MAX_VERSIONS_TO_SEARCH) {
            let (v, txn) = res?;
            // If we've found the beginning of the block, stop
            if matches!(
                txn,
                Transaction::GenesisTransaction(_) | Transaction::BlockMetadata(_)
            ) {
                start_version = Some(v);
                break;
            }
        }

        let mut iter = self.db.iter::<TransactionSchema>(Default::default())?;
        iter.seek(&version)?;
        let mut end_version = None;
        for res in iter.take(MAX_VERSIONS_TO_SEARCH) {
            let (v, txn) = res?;
            match txn {
                Transaction::BlockMetadata(_) => {
                    // If the current version is the beginning of the block, we need to find the
                    // rest of the block
                    if start_version != Some(v) {
                        // This block metadata is the next block, so you need the previous version
                        end_version = Some(v - 1);
                        break;
                    }
                }
                // Every txn up to the end or the block metadata should be included in the block
                _ => {
                    if v == latest_ledger_version {
                        end_version = Some(v);
                        break;
                    }
                }
            }
        }

        // If we've found both, we have the whole block
        match (start_version, end_version) {
            (Some(start), Some(end)) => Ok((start, end)),
            // If we didn't find the end, it's a single transaction block (reconfig)
            (Some(start), None) => Ok((start, start)),
            _ => Err(AptosDbError::NotFound(format!(
                "Block boundaries not found for version {}",
                version
            ))
            .into()),
        }
    }

    /// Save signed transaction at `version`
    pub fn put_transaction(
        &self,
        version: Version,
        transaction: &Transaction,
        cs: &mut ChangeSet,
    ) -> Result<()> {
        if let Transaction::UserTransaction(txn) = transaction {
            cs.batch.put::<TransactionByAccountSchema>(
                &(txn.sender(), txn.sequence_number()),
                &version,
            )?;
        }
        cs.batch
            .put::<TransactionByHashSchema>(&transaction.hash(), &version)?;
        cs.batch.put::<TransactionSchema>(&version, transaction)?;

        Ok(())
    }

    /// Get executed transaction vm output given `version`
    pub fn get_write_set(&self, version: Version) -> Result<WriteSet> {
        self.db.get::<WriteSetSchema>(&version)?.ok_or_else(|| {
            AptosDbError::NotFound(format!("WriteSet at version {}", version)).into()
        })
    }

    /// Get write sets in `[begin_version, end_version)` half-open range.
    ///
    /// N.b. an empty `Vec` is returned when `begin_version == end_version`
    pub fn get_write_sets(
        &self,
        begin_version: Version,
        end_version: Version,
    ) -> Result<Vec<WriteSet>> {
        if begin_version == end_version {
            return Ok(Vec::new());
        }
        ensure!(
            begin_version < end_version,
            "begin_version {} >= end_version {}",
            begin_version,
            end_version
        );

        let mut iter = self.db.iter::<WriteSetSchema>(Default::default())?;
        iter.seek(&begin_version)?;

        let mut ret = Vec::with_capacity((end_version - begin_version) as usize);
        for current_version in begin_version..end_version {
            let (version, write_set) = iter
                .next()
                .transpose()?
                .ok_or_else(|| format_err!("Write set missing for version {}", current_version))?;
            ensure!(
                version == current_version,
                "Write set missing for version {}, got version {}",
                current_version,
                version,
            );
            ret.push(write_set);
        }

        Ok(ret)
    }

    /// Get the first version that write set starts existent.
    pub fn get_first_write_set_version(&self) -> Result<Option<Version>> {
        let mut iter = self.db.iter::<WriteSetSchema>(Default::default())?;
        iter.seek_to_first();
        iter.next().map(|res| res.map(|(v, _)| v)).transpose()
    }

    /// Save executed transaction vm output given `version`
    pub fn put_write_set(
        &self,
        version: Version,
        write_set: &WriteSet,
        cs: &mut ChangeSet,
    ) -> Result<()> {
        cs.batch.put::<WriteSetSchema>(&version, write_set)
    }

    /// Prune the transaction by hash store given a list of transaction
    pub fn prune_transaction_by_hash(
        &self,
        transactions: &[Transaction],
        db_batch: &mut SchemaBatch,
    ) -> anyhow::Result<()> {
        for transaction in transactions {
            db_batch.delete::<TransactionByHashSchema>(&transaction.hash())?;
        }
        Ok(())
    }

    /// Prune the transaction by account store given a list of transaction
    pub fn prune_transaction_by_account(
        &self,
        transactions: &[Transaction],
        db_batch: &mut SchemaBatch,
    ) -> anyhow::Result<()> {
        for transaction in transactions {
            if let Transaction::UserTransaction(txn) = transaction {
                db_batch
                    .delete::<TransactionByAccountSchema>(&(txn.sender(), txn.sequence_number()))?;
            }
        }
        Ok(())
    }

    /// Prune the transaction schema store between a range of version in [begin, end)
    pub fn prune_transaction_schema(
        &self,
        begin: Version,
        end: Version,
        db_batch: &mut SchemaBatch,
    ) -> anyhow::Result<()> {
        for version in begin..end {
            db_batch.delete::<TransactionSchema>(&version)?;
        }
        Ok(())
    }

    /// Prune the transaction schema store between a range of version in [begin, end)
    pub fn prune_transaction_info_schema(
        &self,
        begin: Version,
        end: Version,
        db_batch: &mut SchemaBatch,
    ) -> anyhow::Result<()> {
        for version in begin..end {
            db_batch.delete::<TransactionInfoSchema>(&version)?;
        }
        Ok(())
    }

    /// Prune the transaction schema store between a range of version in [begin, end)
    pub fn prune_write_set(
        &self,
        begin: Version,
        end: Version,
        db_batch: &mut SchemaBatch,
    ) -> anyhow::Result<()> {
        for version in begin..end {
            db_batch.delete::<WriteSetSchema>(&version)?;
        }
        Ok(())
    }

    /// Returns the minimum position node needed to be included in the proof of the leaf index. This
    /// will be the left child of the root if the leaf index is non zero and zero otherwise.
    pub fn get_min_proof_node(&self, leaf_index: u64) -> Position {
        if leaf_index > 0 {
            Position::root_from_leaf_index(leaf_index).left_child()
        } else {
            // Handle this as a special case when min_readable_version is 0
            Position::root_from_leaf_index(0)
        }
    }
}

pub struct TransactionIter<'a> {
    inner: SchemaIterator<'a, TransactionSchema>,
    expected_next_version: Version,
    end_version: Version,
}

impl<'a> TransactionIter<'a> {
    fn next_impl(&mut self) -> Result<Option<Transaction>> {
        if self.expected_next_version >= self.end_version {
            return Ok(None);
        }

        let ret = match self.inner.next().transpose()? {
            Some((version, transaction)) => {
                ensure!(
                    version == self.expected_next_version,
                    "Transaction versions are not consecutive.",
                );
                self.expected_next_version += 1;
                Some(transaction)
            }
            None => None,
        };

        Ok(ret)
    }
}

impl<'a> Iterator for TransactionIter<'a> {
    type Item = Result<Transaction>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_impl().transpose()
    }
}

// TODO(philiphayes): this will need to change to support CRSNs
// (Conflict-Resistant Sequence Numbers)[https://github.com/diem/dip/blob/main/dips/dip-168.md].
//
// It depends on the implementation details, but we'll probably index by _requested_
// transaction sequence number rather than committed account sequence number.
// This would mean the property: `seq_num_{i+1} == seq_num_{i} + 1` would no longer
// be guaranteed and the check should be removed.
//
// This index would also no longer iterate over an account's transactions in
// committed order, meaning the outer method would need to overread by
// `CRSN_WINDOW_SIZE`, sort by version, and take only `limit` entries to get
// at most `limit` transactions in committed order. Alternatively, add another
// index for scanning an accounts transactions in committed order, e.g.,
// `(AccountAddress, Version) -> SeqNum`.

pub struct AccountTransactionVersionIter<'a> {
    inner: SchemaIterator<'a, TransactionByAccountSchema>,
    address: AccountAddress,
    expected_next_seq_num: Option<u64>,
    end_seq_num: u64,
    prev_version: Option<Version>,
    ledger_version: Version,
}

impl<'a> AccountTransactionVersionIter<'a> {
    fn next_impl(&mut self) -> Result<Option<(u64, Version)>> {
        Ok(match self.inner.next().transpose()? {
            Some(((address, seq_num), version)) => {
                // No more transactions sent by this account.
                if address != self.address {
                    return Ok(None);
                }
                if seq_num >= self.end_seq_num {
                    return Ok(None);
                }

                // Ensure seq_num_{i+1} == seq_num_{i} + 1
                if let Some(expected_seq_num) = self.expected_next_seq_num {
                    ensure!(
                        seq_num == expected_seq_num,
                        "DB corruption: account transactions sequence numbers are not contiguous: \
                     actual: {}, expected: {}",
                        seq_num,
                        expected_seq_num,
                    );
                };

                // Ensure version_{i+1} > version_{i}
                if let Some(prev_version) = self.prev_version {
                    ensure!(
                        prev_version < version,
                        "DB corruption: account transaction versions are not strictly increasing: \
                         previous version: {}, current version: {}",
                        prev_version,
                        version,
                    );
                }

                // No more transactions (in this view of the ledger).
                if version > self.ledger_version {
                    return Ok(None);
                }

                self.expected_next_seq_num = Some(seq_num + 1);
                self.prev_version = Some(version);
                Some((seq_num, version))
            }
            None => None,
        })
    }
}

impl<'a> Iterator for AccountTransactionVersionIter<'a> {
    type Item = Result<(u64, Version)>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_impl().transpose()
    }
}

#[cfg(test)]
mod test;
