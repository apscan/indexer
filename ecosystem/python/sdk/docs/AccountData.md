# AccountData

#### Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**sequence_number** | **str** | A string containing a 64-bit unsigned integer.  We represent u64 values as a string to ensure compatability with languages such as JavaScript that do not parse u64s in JSON natively.  | 
**authentication_key** | **str** | All bytes (Vec&lt;u8&gt;) data is represented as hex-encoded string prefixed with &#x60;0x&#x60; and fulfilled with two hex digits per byte.  Unlike the &#x60;Address&#x60; type, HexEncodedBytes will not trim any zeros.  | 
**any string name** | **bool, date, datetime, dict, float, int, list, str, none_type** | any string name can be used but the value must be the correct type | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

