// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use crate::{event_store::EventStore, pruner::db_sub_pruner::DBSubPruner, AptosDB};
use schemadb::SchemaBatch;
use std::sync::Arc;

pub struct EventStorePruner {
    db: Arc<AptosDB>,
}

impl DBSubPruner for EventStorePruner {
    fn prune(
        &self,
        db_batch: &mut SchemaBatch,
        min_readable_version: u64,
        target_version: u64,
    ) -> anyhow::Result<()> {
        self.db
            .prune_events(min_readable_version, target_version, db_batch)?;
        Ok(())
    }
}

impl EventStorePruner {
    pub(in crate::pruner) fn new(db: Arc<AptosDB>) -> Self {
        EventStorePruner { db }
    }
}
