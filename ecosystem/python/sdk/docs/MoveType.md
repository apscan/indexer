# MoveType

String representation of an on-chain Move type tag that is exposed in transaction payload.     Values:       - bool       - u8       - u64       - u128       - address       - signer       - vector: `vector<{non-reference MoveTypeId}>`       - struct: `{address}::{module_name}::{struct_name}::<{generic types}>`      Vector type value examples:       - `vector<u8>`       - `vector<vector<u64>>`       - `vector<0x1::coin::CoinStore<0x1::aptos_coin::AptosCoin>>`      Struct type value examples:       - `0x1::coin::CoinStore<0x1::aptos_coin::AptosCoin>       - `0x1::account::Account`      Note:       1. Empty chars should be ignored when comparing 2 struct tag ids.       2. When used in an URL path, should be encoded by url-encoding (AKA percent-encoding). 

Type | Description | Notes
------------- | ------------- | -------------
**str** | String representation of an on-chain Move type tag that is exposed in transaction payload.     Values:       - bool       - u8       - u64       - u128       - address       - signer       - vector: &#x60;vector&lt;{non-reference MoveTypeId}&gt;&#x60;       - struct: &#x60;{address}::{module_name}::{struct_name}::&lt;{generic types}&gt;&#x60;      Vector type value examples:       - &#x60;vector&lt;u8&gt;&#x60;       - &#x60;vector&lt;vector&lt;u64&gt;&gt;&#x60;       - &#x60;vector&lt;0x1::coin::CoinStore&lt;0x1::aptos_coin::AptosCoin&gt;&gt;&#x60;      Struct type value examples:       - &#x60;0x1::coin::CoinStore&lt;0x1::aptos_coin::AptosCoin&gt;       - &#x60;0x1::account::Account&#x60;      Note:       1. Empty chars should be ignored when comparing 2 struct tag ids.       2. When used in an URL path, should be encoded by url-encoding (AKA percent-encoding).  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

