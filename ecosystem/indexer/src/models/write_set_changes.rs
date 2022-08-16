// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0
#![allow(clippy::extra_unused_lifetimes)]
use crate::{models::transactions::Transaction, schema::{write_set_changes, resource_changes, module_changes, table_item_changes}};
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

pub struct WriteSetChangePlural {
    pub resource_changes: Vec<ResourceChange>,
    pub module_changes: Vec<ModuleChange>,
    pub table_item_changes: Vec<TableItemChange>
}

impl WriteSetChangePlural {
    pub fn from_write_set_changes(
        transaction_version: i64,
        write_set_changes: &[APIWriteSetChange],
    ) -> Self {
        let mut resource_changes = Vec::new();
        let mut module_changes = Vec::new();
        let mut table_item_changes = Vec::new();
        for (id, change) in write_set_changes.iter().enumerate() {
            match change {
                APIWriteSetChange::DeleteModule(delete_module) => {
                    module_changes.push(ModuleChange::from_delete_change(transaction_version, id as i32, delete_module.clone()))
                }
                APIWriteSetChange::WriteModule(write_module) => {
                    module_changes.push(ModuleChange::from_write_change(transaction_version, id as i32, write_module.clone()))
                }
                APIWriteSetChange::DeleteResource(delete_resouce) => {
                    resource_changes.push(ResourceChange::from_delete_change(transaction_version, id as i32, delete_resouce.clone()))
                }                
                APIWriteSetChange::WriteResource(write_resouce) => {
                    resource_changes.push(ResourceChange::from_write_change(transaction_version, id as i32, write_resouce.clone()))
                }
                APIWriteSetChange::DeleteTableItem(delete_table_item) => {
                    table_item_changes.push(TableItemChange::from_delete_change(transaction_version, id as i32, delete_table_item.clone()))
                }
                APIWriteSetChange::WriteTableItem(write_table_item) => {
                    table_item_changes.push(TableItemChange::from_write_change(transaction_version, id as i32, write_table_item.clone()))
                }                                                     
            }
        }
    Self{resource_changes, module_changes, table_item_changes}
    }

    pub fn extend(&mut self, new_changes_plural : Self) -> &Self {
        self.module_changes.extend(new_changes_plural.module_changes);
        self.resource_changes.extend(new_changes_plural.resource_changes);
        self.table_item_changes.extend(new_changes_plural.table_item_changes);
        self
    }

    pub fn new() -> Self {
        Self { resource_changes: Vec::new(), module_changes: Vec::new(), table_item_changes: Vec::new() }
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
    pub move_resource_address: String,
    pub move_resource_module: String,
    pub move_resource_name: String,
    pub move_resource_generic_type_params: serde_json::Value,
    pub move_resource_data: serde_json::Value
}

impl ResourceChange{
    pub fn from_write_change(
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
            move_resource_address: write_resource.data.typ.address.to_string(),
            move_resource_module: write_resource.data.typ.module.to_string(),
            move_resource_name: write_resource.data.typ.name.to_string(),
            move_resource_generic_type_params: serde_json::to_value(write_resource.data.typ.generic_type_params).unwrap(),
            move_resource_data: serde_json::to_value(write_resource.data.data).unwrap()
        }
    }

    pub fn from_delete_change(
        transaction_version: i64,
        transaction_index: i32,
        delete_resource: DeleteResource
    ) -> Self {
        ResourceChange {
            transaction_version,
            transaction_index,
            is_write: false,
            address: delete_resource.address.to_string(),
            state_key_hash: delete_resource.state_key_hash.clone(),
            move_resource_address: delete_resource.resource.address.to_string(),
            move_resource_module: delete_resource.resource.module.to_string(),
            move_resource_name: delete_resource.resource.name.to_string(),
            move_resource_generic_type_params: serde_json::to_value(delete_resource.resource.generic_type_params).unwrap(),
            move_resource_data: Default::default()
        }
    }
}

#[derive(AsChangeset, Associations, Debug, Identifiable, Insertable, Queryable, Serialize)]
#[diesel(table_name = "module_change")]
#[belongs_to(Transaction, foreign_key = "transaction_version")]
#[primary_key(transaction_version, state_key_hash)]
pub struct ModuleChange {
    pub transaction_version: i64,
    pub transaction_index: i32,
    pub is_write: bool,
    pub address: String,
    pub state_key_hash: String,
    pub move_module_address: String,
    pub move_module_name: String,
    pub move_module_bytecode: String,
    pub move_module_abi: serde_json::Value,
}

impl ModuleChange{
    pub fn from_write_change(
        transaction_version: i64,
        transaction_index: i32,
        write_module: WriteModule
    ) -> Self {
        let abi = write_module.data.clone().try_parse_abi().unwrap();
        ModuleChange {
            transaction_version,
            transaction_index,
            is_write: true,
            address: write_module.address.to_string(),
            state_key_hash: write_module.state_key_hash.clone(),
            move_module_address: match &abi.abi {
                None => Default::default(),
                Some(abi_data) => abi_data.address.to_string()
            },
            move_module_name: match &abi.abi {
                None => Default::default(),
                Some(abi_data) => abi_data.name.to_string()
            },
            move_module_bytecode: write_module.data.bytecode.to_string(),
            move_module_abi: match &abi.abi {
                None => Default::default(),
                Some(abi_data) => serde_json::to_value(abi_data).unwrap()
            }
        }
    }

    pub fn from_delete_change(
        transaction_version: i64,
        transaction_index: i32,
        delete_module: DeleteModule
    ) -> Self {
        ModuleChange {
            transaction_version,
            transaction_index,
            is_write: false,
            address: delete_module.address.to_string(),
            state_key_hash: delete_module.state_key_hash.clone(),
            move_module_address: delete_module.module.address.to_string(),
            move_module_name: delete_module.module.name.to_string(),
            move_module_bytecode: Default::default(),
            move_module_abi: Default::default()
        }
    }
}

#[derive(AsChangeset, Associations, Debug, Identifiable, Insertable, Queryable, Serialize)]
#[diesel(table_name = "table_item_changes")]
#[belongs_to(Transaction, foreign_key = "transaction_version")]
#[primary_key(transaction_version, state_key_hash)]
pub struct TableItemChange {
    pub transaction_version: i64,
    pub transaction_index: i32,
    pub is_write: bool,
    pub state_key_hash: String,
    pub handle: String,
    pub key: String,
    pub value: String,
    pub table_data_key: serde_json::Value,
    pub table_data_key_type: String,
    pub table_data_value: serde_json::Value,
    pub table_data_value_type: String,
}

impl TableItemChange{
    pub fn from_write_change(
        transaction_version: i64,
        transaction_index: i32,
        write_table_item: WriteTableItem
    ) -> Self {
        let table_data = write_table_item.data.clone();
        TableItemChange {
            transaction_version,
            transaction_index,
            is_write: true,
            state_key_hash: write_table_item.state_key_hash.clone(),
            handle: write_table_item.handle.to_string(),
            key: write_table_item.key.to_string(),
            value: write_table_item.value.to_string(),
            table_data_key: match &table_data {
                None => Default::default(),
                Some(data) => serde_json::to_value(&data.key).unwrap()
            },
            table_data_key_type: match &table_data {
                None => Default::default(),
                Some(data) => data.key_type.to_string()
            },
            table_data_value: match &table_data {
                None => Default::default(),
                Some(data) => serde_json::to_value(&data.value).unwrap()
            },
            table_data_value_type: match &table_data {
                None => Default::default(),
                Some(data) => data.value_type.to_string()
            },
        }
    }

    pub fn from_delete_change(
        transaction_version: i64,
        transaction_index: i32,
        delete_table_item: DeleteTableItem
    ) -> Self {
        let table_data = delete_table_item.data.clone();
        TableItemChange {
            transaction_version,
            transaction_index,
            is_write: false,
            state_key_hash: delete_table_item.state_key_hash.clone(),
            handle: delete_table_item.handle.to_string(),
            key: delete_table_item.key.to_string(),
            value: Default::default(),
            table_data_key: match &table_data {
                None => Default::default(),
                Some(data) => serde_json::to_value(&data.key).unwrap()
            },
            table_data_key_type: match &table_data {
                None => Default::default(),
                Some(data) => data.key_type.to_string()
            },
            table_data_value: Default::default(),
            table_data_value_type: Default::default(),
        }
    }
    }

// Prevent conflicts with other things named `WriteSetChange`
pub type WriteSetChangeModel = WriteSetChange;
