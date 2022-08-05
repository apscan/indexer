# UserTransaction

#### Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**version** | **str** | A string containing a 64-bit unsigned integer.  We represent u64 values as a string to ensure compatability with languages such as JavaScript that do not parse u64s in JSON natively.  | 
**hash** | **str** |  | 
**state_root_hash** | **str** |  | 
**event_root_hash** | **str** |  | 
**gas_used** | **str** | A string containing a 64-bit unsigned integer.  We represent u64 values as a string to ensure compatability with languages such as JavaScript that do not parse u64s in JSON natively.  | 
**success** | **bool** |  | 
**vm_status** | **str** |  | 
**accumulator_root_hash** | **str** |  | 
**changes** | **[WriteSetChange]** |  | 
**sender** | **str** | Hex encoded 32 byte Aptos account address | 
**sequence_number** | **str** | A string containing a 64-bit unsigned integer.  We represent u64 values as a string to ensure compatability with languages such as JavaScript that do not parse u64s in JSON natively.  | 
**max_gas_amount** | **str** | A string containing a 64-bit unsigned integer.  We represent u64 values as a string to ensure compatability with languages such as JavaScript that do not parse u64s in JSON natively.  | 
**gas_unit_price** | **str** | A string containing a 64-bit unsigned integer.  We represent u64 values as a string to ensure compatability with languages such as JavaScript that do not parse u64s in JSON natively.  | 
**expiration_timestamp_secs** | **str** | A string containing a 64-bit unsigned integer.  We represent u64 values as a string to ensure compatability with languages such as JavaScript that do not parse u64s in JSON natively.  | 
**payload** | [**TransactionPayload**](TransactionPayload.md) |  | 
**signature** | [**TransactionSignature**](TransactionSignature.md) |  | [optional] 
**events** | **[Event]** |  | 
**timestamp** | **str** | A string containing a 64-bit unsigned integer.  We represent u64 values as a string to ensure compatability with languages such as JavaScript that do not parse u64s in JSON natively.  | 
**any string name** | **bool, date, datetime, dict, float, int, list, str, none_type** | any string name can be used but the value must be the correct type | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

