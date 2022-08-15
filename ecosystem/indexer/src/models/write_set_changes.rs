// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0
#![allow(clippy::extra_unused_lifetimes)]
use crate::{models::transactions::Transaction, schema::{write_set_changes, resource_changes}};
use aptos_rest_client::aptos_api_types::{
    DeleteModule, DeleteResource, DeleteTableItem, WriteModule, WriteResource,
    WriteSetChange as APIWriteSetChange, WriteTableItem,
};
use serde::Serialize;
use serde_json::json;

#[derive(AsChangeset, Associations, Debug, Identifiable, Insertable, Queryable, Serialize)]
#[diesel(table_name = "write_set_changes")]
#[belongs_to(Transaction, foreign_key = "transaction_version")]
#[primary_key(transaction_version, state_key_hash)]
pub struct WriteSetChange {
    pub transaction_version: i64,
    pub state_key_hash: String,
    #[diesel(column_name = type)]
    pub change_type: String,
    pub address: String,
    pub module: serde_json::Value,
    pub resource: serde_json::Value,
    pub data: serde_json::Value,

    // Default time columns
    pub inserted_at: chrono::NaiveDateTime,
}

impl WriteSetChange {
    pub fn from_write_set_change(
        transaction_version: i64,
        write_set_change: &APIWriteSetChange,
    ) -> Self {
        match write_set_change {
            APIWriteSetChange::DeleteModule(DeleteModule {
                address,
                state_key_hash,
                module,
            }) => WriteSetChange {
                transaction_version,
                state_key_hash: state_key_hash.clone(),
                change_type: write_set_change.type_str().to_string(),
                address: address.to_string(),
                module: serde_json::to_value(module).unwrap(),
                resource: Default::default(),
                data: Default::default(),
                inserted_at: chrono::Utc::now().naive_utc(),
            },
            APIWriteSetChange::DeleteResource(DeleteResource {
                address,
                state_key_hash,
                resource,
            }) => WriteSetChange {
                transaction_version,
                state_key_hash: state_key_hash.clone(),
                change_type: write_set_change.type_str().to_string(),
                address: address.to_string(),
                module: Default::default(),
                resource: serde_json::to_value(resource).unwrap(),
                data: Default::default(),
                inserted_at: chrono::Utc::now().naive_utc(),
            },
            APIWriteSetChange::DeleteTableItem(DeleteTableItem {
                state_key_hash,
                handle,
                key,
                ..
            }) => WriteSetChange {
                transaction_version,
                state_key_hash: state_key_hash.clone(),
                change_type: write_set_change.type_str().to_string(),
                address: "".to_owned(),
                module: Default::default(),
                resource: Default::default(),
                data: json!({
                    "handle": handle,
                    "key": key,
                }),
                inserted_at: chrono::Utc::now().naive_utc(),
            },
            APIWriteSetChange::WriteModule(WriteModule {
                address,
                state_key_hash,
                data,
            }) => 
            {
                println!("{}", serde_json::to_value(data.clone().try_parse_abi().unwrap()).unwrap());
                WriteSetChange {
                transaction_version,
                state_key_hash: state_key_hash.clone(),
                change_type: write_set_change.type_str().to_string(),
                address: address.to_string(),
                module: Default::default(),
                resource: Default::default(),
                data: serde_json::to_value(data).unwrap(),
                inserted_at: chrono::Utc::now().naive_utc(),
            }},
            APIWriteSetChange::WriteResource(WriteResource {
                address,
                state_key_hash,
                data,
            }) => WriteSetChange {
                transaction_version,
                state_key_hash: state_key_hash.clone(),
                change_type: write_set_change.type_str().to_string(),
                address: address.to_string(),
                module: Default::default(),
                resource: Default::default(),
                data: serde_json::to_value(data).unwrap(),
                inserted_at: chrono::Utc::now().naive_utc(),
            },
            APIWriteSetChange::WriteTableItem(WriteTableItem {
                state_key_hash,
                handle,
                key,
                value,
                ..
            }) => WriteSetChange {
                transaction_version,
                state_key_hash: state_key_hash.clone(),
                change_type: write_set_change.type_str().to_string(),
                address: "".to_owned(),
                module: Default::default(),
                resource: Default::default(),
                data: json!({
                    "handle": handle,
                    "key": key,
                    "value": value,
                }),
                inserted_at: chrono::Utc::now().naive_utc(),
            },
        }
    }

    pub fn from_write_set_changes(
        version: i64,
        write_set_changes: &[APIWriteSetChange],
    ) -> Option<Vec<Self>> {
        if write_set_changes.is_empty() {
            return None;
        }
        Some(
            write_set_changes
                .iter()
                .map(|write_set_change| {
                    Self::from_write_set_change( version, write_set_change)
                })
                .collect::<Vec<WriteSetChangeModel>>(),
        )
    }
}

#[derive(AsChangeset, Associations, Debug, Identifiable, Insertable, Queryable, Serialize)]
#[diesel(table_name = "resource_changes")]
#[belongs_to(Transaction, foreign_key = "transaction_version")]
#[primary_key(transaction_version, state_key_hash)]
pub struct ResourceChange {
    pub transaction_version: i64,
    pub transaction_index: i32,
    pub is_write: bool,
    pub address: String,
    pub state_key_hash: String,
    pub move_module_tag: serde_json::Value,
    pub move_module_value: serde_json::Value,
}

impl ResourceChange{
    pub fn from_write_resource(
        transaction_version: i64,
        transaction_index: i32,
        write_resource: WriteResource
    ) -> Self {
        ResourceChange {
            transaction_version,
            transaction_index,
            is_write: true,
            address: write_resource.address.to_string(),
            state_key_hash: write_resource.state_key_hash.clone(),
            move_module_tag: serde_json::to_value(write_resource.data.typ).unwrap(),
            move_module_value: serde_json::to_value(write_resource.data.data).unwrap()
        }
    }

    pub fn from_delete_resource(
        transaction_version: i64,
        transaction_index: i32,
        delete_resource: DeleteResource
    ) -> Self {
        ResourceChange {
            transaction_version,
            transaction_index,
            is_write: true,
            address: delete_resource.address.to_string(),
            state_key_hash: delete_resource.state_key_hash.clone(),
            move_module_tag: serde_json::to_value(delete_resource.resource).unwrap(),
            move_module_value: Default::default()
        }
    }
}


// Prevent conflicts with other things named `WriteSetChange`
pub type WriteSetChangeModel = WriteSetChange;
