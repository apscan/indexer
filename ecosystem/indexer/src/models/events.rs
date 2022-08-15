// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0
#![allow(clippy::extra_unused_lifetimes)]
use crate::{models::transactions::Transaction, schema::{events, blocks}};
use aptos_rest_client::aptos_api_types::Event as APIEvent;
use serde::{Deserialize, Serialize};

#[derive(Associations, Debug, Identifiable, Insertable, Queryable, Serialize)]
#[diesel(table_name = "events")]
#[belongs_to(Transaction, foreign_key = "transaction_version")]
#[primary_key(key, sequence_number)]
pub struct Event {
    pub transaction_version: i64,
    pub key: String,
    pub sequence_number: i64,
    #[diesel(column_name = type)]
    pub type_: String,
    pub data: serde_json::Value,

    // Default time columns
    pub inserted_at: chrono::NaiveDateTime,
}

impl Event {
    pub fn from_event(transaction_version: i64, event: &APIEvent) -> Self {
        Event {
            transaction_version,
            key: event.key.to_string(),
            sequence_number: event.sequence_number.0 as i64,
            type_: event.typ.to_string(),
            data: event.data.clone(),
            inserted_at: chrono::Utc::now().naive_utc(),
        }
    }

    pub fn from_events(transaction_version: i64, events: &[APIEvent]) -> Vec<Self> {
        events
            .iter()
            .map(|event| Self::from_event(transaction_version.clone(), event))
            .collect::<Vec<EventModel>>()
    }
}

// Prevent conflicts with other things named `Event`
pub type EventModel = Event;
