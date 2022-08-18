// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0
#![allow(clippy::extra_unused_lifetimes)]
use crate::{models::transactions::Transaction, schema::{events, event_keys}};
use aptos_rest_client::aptos_api_types::Event as APIEvent;
use serde::Serialize;

#[derive(Associations, Debug, Identifiable, Insertable, Queryable, Serialize)]
#[diesel(table_name = "events")]
#[belongs_to(Transaction, foreign_key = "transaction_version")]
#[primary_key(key, sequence_number)]
pub struct Event {
    pub transaction_version: i64,
    pub transaction_index: i32,
    pub key: String,
    pub sequence_number: i64,
    #[diesel(column_name = type)]
    pub type_: String,
    pub data: serde_json::Value,
}

impl Event {
    pub fn from_event(transaction_version: i64, transaction_index: i32, event: &APIEvent) -> Self {
        Event {
            transaction_version,
            transaction_index,
            key: event.key.to_string(),
            sequence_number: event.sequence_number.0 as i64,
            type_: event.typ.to_string(),
            data: event.data.clone(),
        }
    }

    pub fn from_events(transaction_version: i64, events: &[APIEvent]) -> Vec<Self> {
        events.iter()
            .enumerate()
            .map(|(idx, event)| Self::from_event(transaction_version.clone(), idx as i32, event))
            .collect()
    }
}

#[derive(Associations, Debug, Identifiable, Insertable, Queryable, Serialize)]
#[diesel(table_name = "event_keys")]
#[primary_key(key)]
pub struct EventKey {
    pub key: String,
    pub account: String,
    pub creation_num : i64,
    pub move_type : serde_json::Value,
}

impl EventKey {
    pub fn from_event( event: &APIEvent) -> Self {
        EventKey {
            key: event.key.to_string(),
            account: event.key.0.get_creator_address().to_string(),
            creation_num: event.key.0.get_creation_number() as i64,
            move_type: serde_json::to_value(&event.typ).unwrap()
        }
    }

    pub fn from_events(events: &[APIEvent]) -> Vec<Self> {
        events.iter()
            .map(|event| Self::from_event(event))
            .collect()
    }
}

pub struct EventModelPlural {
    pub events: Vec<Event>,
    pub event_keys: Vec<EventKey>,
}

impl EventModelPlural {
    pub fn new() -> Self {
        Self { events: Vec::new(), event_keys: Vec::new() }
    }

    pub fn from_events(transaction_version : i64, events: &[APIEvent]) -> Self {
        Self {
            events : Event::from_events(transaction_version, events),
            event_keys: EventKey::from_events(events)
        }
    }

    pub fn extend(&mut self, event_model_plural : EventModelPlural) {
        self.events.extend(event_model_plural.events);
        self.event_keys.extend(event_model_plural.event_keys);
    }
}

pub type EventModel = Event;
// Prevent conflicts with other things named `Event`
