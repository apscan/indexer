use crate::{schema::{script_write_set_payloads, direct_write_set_payloads, script_function_payloads, module_bundle_payloads, script_payloads}};
use aptos_rest_client::aptos_api_types::{TransactionPayload, WriteSet};
use serde::{Deserialize, Serialize};


#[derive(Associations, Debug, Identifiable, Insertable, Queryable, Serialize, Deserialize)]
#[diesel(table_name = "script_write_set_payloads")]
#[primary_key(transaction_version)]
pub struct ScriptWriteSetPayload {
    pub transaction_version : i64,
    pub execute_as : String,
    pub code : String,
    pub abi : serde_json::Value,
    pub type_arguments : serde_json::Value,
    pub arguments : serde_json::Value,
}

#[derive(Associations, Debug, Identifiable, Insertable, Queryable, Serialize, Deserialize)]
#[diesel(table_name = "direct_write_set_payloads")]
#[primary_key(transaction_version)]
pub struct DirectWriteSetPayload {
    pub transaction_version : i64,
    pub events: serde_json::Value,
    pub changes : serde_json::Value,
}

#[derive(Associations, Debug, Identifiable, Insertable, Queryable, Serialize, Deserialize)]
#[diesel(table_name = "script_function_payload")]
#[primary_key(transaction_version)]
pub struct ScriptFunctionPayload {
    pub transaction_version : i64,
    pub script_function_module_address : String,
    pub script_function_module_name : String,
    pub script_function_name : String,
    pub type_arguments : serde_json::Value,
    pub arguments : serde_json::Value,
}

#[derive(Associations, Debug, Identifiable, Insertable, Queryable, Serialize, Deserialize)]
#[diesel(table_name = "module_bundle_payloads")]
#[primary_key(transaction_version)]
pub struct ModuleBundlePayload {
    pub transaction_version : i64,
    pub modules: serde_json::Value,
}

#[derive(Associations, Debug, Identifiable, Insertable, Queryable, Serialize, Deserialize)]
#[diesel(table_name = "script_payloads")]
#[primary_key(transaction_version)]
pub struct ScriptPayload {
    pub transaction_version : i64,
    pub code : String,
    pub abi : serde_json::Value,
    pub type_arguments : serde_json::Value,
    pub arguments : serde_json::Value,
}

pub enum TransactionPayloadModel {
    ScriptWriteSetPayload(ScriptWriteSetPayload),
    DirectWriteSetPayload(DirectWriteSetPayload),
    ScriptFunctionPayload(ScriptFunctionPayload),
    ModuleBundlePayload(ModuleBundlePayload),
    ScriptPayload(ScriptPayload),
}

impl TransactionPayloadModel {
    pub fn from_transaction_payload(transaction_version: i64, payload : TransactionPayload) -> Self {
        match payload {
            TransactionPayload::ScriptFunctionPayload(payload_data) => TransactionPayloadModel::ScriptFunctionPayload(ScriptFunctionPayload{
                transaction_version,
                script_function_module_address : payload_data.function.module.address.to_string(),
                script_function_module_name : payload_data.function.module.name.to_string(),
                script_function_name : payload_data.function.name.to_string(),
                type_arguments : serde_json::to_value(payload_data.type_arguments).unwrap(),
                arguments : serde_json::to_value(payload_data.arguments).unwrap(),
            }),
            TransactionPayload::ScriptPayload(payload_data) => TransactionPayloadModel::ScriptPayload(ScriptPayload {
                transaction_version,
                code: payload_data.code.bytecode.to_string(),
                abi: match payload_data.code.try_parse_abi().abi {
                    None => Default::default(),
                    Some(abi_data) => serde_json::to_value(abi_data).unwrap()
                },
                type_arguments: serde_json::to_value(payload_data.type_arguments).unwrap(),
                arguments: serde_json::to_value(payload_data.arguments).unwrap(),
            }),
            TransactionPayload::ModuleBundlePayload(payload_data) => TransactionPayloadModel::ModuleBundlePayload(ModuleBundlePayload{
                transaction_version,
                modules: serde_json::to_value(payload_data.modules).unwrap()
            }),
            TransactionPayload::WriteSetPayload(payload_data) => match payload_data.write_set {
                WriteSet::ScriptWriteSet(script_write_set) => TransactionPayloadModel::ScriptWriteSetPayload(ScriptWriteSetPayload{
                    transaction_version,
                    execute_as: script_write_set.execute_as.to_string(),
                    code: script_write_set.script.code.bytecode.to_string(),
                    abi: match script_write_set.script.code.try_parse_abi().abi {
                        Some(abi_data) => serde_json::to_value(abi_data).unwrap(),
                        None => Default::default(),
                    },
                    type_arguments: serde_json::to_value(script_write_set.script.type_arguments).unwrap(),
                    arguments: serde_json::to_value(script_write_set.script.arguments).unwrap(),
                }),
                WriteSet::DirectWriteSet(direct_write_set) => TransactionPayloadModel::DirectWriteSetPayload(DirectWriteSetPayload{
                    transaction_version,
                    events: serde_json::to_value(direct_write_set.events).unwrap(),
                    changes: serde_json::to_value(direct_write_set.changes).unwrap(),
                }),
            },
        }
    }
 }

pub struct TransactionPayloadPlural {
    pub script_write_set_payloads : Vec<ScriptWriteSetPayload>,
    pub direct_write_set_payloads : Vec<DirectWriteSetPayload>,
    pub script_function_payloads : Vec<ScriptFunctionPayload>,
    pub module_bundle_payloads : Vec<ModuleBundlePayload>,
    pub script_payloads : Vec<ScriptPayload>
}

impl TransactionPayloadPlural {
    pub fn new() -> Self {
        Self { script_write_set_payloads: Vec::new(), 
            direct_write_set_payloads: Vec::new(), 
            script_function_payloads: Vec::new(), 
            module_bundle_payloads: Vec::new(), 
            script_payloads: Vec::new() }
    }
    pub fn append(&mut self, payload : TransactionPayloadModel) {
        match payload {
            TransactionPayloadModel::ScriptWriteSetPayload(payload_data) => self.script_write_set_payloads.push(payload_data),
            TransactionPayloadModel::DirectWriteSetPayload(payload_data) => self.direct_write_set_payloads.push(payload_data),
            TransactionPayloadModel::ScriptFunctionPayload(payload_data) => self.script_function_payloads.push(payload_data),
            TransactionPayloadModel::ModuleBundlePayload(payload_data) => self.module_bundle_payloads.push(payload_data),
            TransactionPayloadModel::ScriptPayload(payload_data) => self.script_payloads.push(payload_data),
        };
    }
}

pub type ScriptWriteSetPayloadModel = ScriptWriteSetPayload;
pub type DirectWriteSetPayloadModel = DirectWriteSetPayload;
pub type ScriptFunctionPayloadModel = ScriptFunctionPayload;
