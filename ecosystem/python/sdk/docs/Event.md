# Event

#### Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | **str** | Event key is a global index for an event stream.  It is hex-encoded BCS bytes of &#x60;EventHandle&#x60; &#x60;guid&#x60; field value, which is a combination of a &#x60;uint64&#x60; creation number and account address (without trimming leading zeros).  For example, event key &#x60;0x000000000000000088fbd33f54e1126269769780feb24480428179f552e2313fbe571b72e62a1ca1&#x60; is combined by the following 2 parts:   1. &#x60;0000000000000000&#x60;: &#x60;uint64&#x60; representation of &#x60;0&#x60;.   2. &#x60;88fbd33f54e1126269769780feb24480428179f552e2313fbe571b72e62a1ca1&#x60;: 32 bytes of account address.  | 
**sequence_number** | **str** | A string containing a 64-bit unsigned integer.  We represent u64 values as a string to ensure compatability with languages such as JavaScript that do not parse u64s in JSON natively.  | 
**type** | [**MoveType**](MoveType.md) |  | 
**data** | **bool, date, datetime, dict, float, int, list, str, none_type** |  | 
**any string name** | **bool, date, datetime, dict, float, int, list, str, none_type** | any string name can be used but the value must be the correct type | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

