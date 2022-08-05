# AptosError

This is the generic struct we use for all API errors, it contains a string message and an Aptos API specific error code.

#### Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**message** | **str** |  | 
**error_code** | [**AptosErrorCode**](AptosErrorCode.md) |  | [optional] 
**aptos_ledger_version** | **str** | A string containing a 64-bit unsigned integer.  We represent u64 values as a string to ensure compatability with languages such as JavaScript that do not parse u64s in JSON natively.  | [optional] 
**any string name** | **bool, date, datetime, dict, float, int, list, str, none_type** | any string name can be used but the value must be the correct type | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

