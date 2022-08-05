# IndexResponse

The struct holding all data returned to the client by the index endpoint (i.e., GET \"/\").

#### Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**chain_id** | **int** |  | 
**epoch** | **str** | A string containing a 64-bit unsigned integer.  We represent u64 values as a string to ensure compatability with languages such as JavaScript that do not parse u64s in JSON natively.  | 
**ledger_version** | **str** | A string containing a 64-bit unsigned integer.  We represent u64 values as a string to ensure compatability with languages such as JavaScript that do not parse u64s in JSON natively.  | 
**oldest_ledger_version** | **str** | A string containing a 64-bit unsigned integer.  We represent u64 values as a string to ensure compatability with languages such as JavaScript that do not parse u64s in JSON natively.  | 
**ledger_timestamp** | **str** | A string containing a 64-bit unsigned integer.  We represent u64 values as a string to ensure compatability with languages such as JavaScript that do not parse u64s in JSON natively.  | 
**node_role** | [**RoleType**](RoleType.md) |  | 
**any string name** | **bool, date, datetime, dict, float, int, list, str, none_type** | any string name can be used but the value must be the correct type | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

