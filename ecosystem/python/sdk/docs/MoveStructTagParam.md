# MoveStructTagParam

String representation of a MoveStructTag (on-chain Move struct type). This exists so you can specify MoveStructTags as path / query parameters, e.g. for get_events_by_event_handle.  It is a combination of:   1. `move_module_address`, `module_name` and `struct_name`, all joined by `::`   2. `struct generic type parameters` joined by `, `  Examples:   * `0x1::coin::CoinStore<0x1::aptos_coin::AptosCoin>`   * `0x1::account::Account`  Note:   1. Empty chars should be ignored when comparing 2 struct tag ids.   2. When used in an URL path, should be encoded by url-encoding (AKA percent-encoding).  See [doc](https://aptos.dev/concepts/basics-accounts) for more details. 

Type | Description | Notes
------------- | ------------- | -------------
**str** | String representation of a MoveStructTag (on-chain Move struct type). This exists so you can specify MoveStructTags as path / query parameters, e.g. for get_events_by_event_handle.  It is a combination of:   1. &#x60;move_module_address&#x60;, &#x60;module_name&#x60; and &#x60;struct_name&#x60;, all joined by &#x60;::&#x60;   2. &#x60;struct generic type parameters&#x60; joined by &#x60;, &#x60;  Examples:   * &#x60;0x1::coin::CoinStore&lt;0x1::aptos_coin::AptosCoin&gt;&#x60;   * &#x60;0x1::account::Account&#x60;  Note:   1. Empty chars should be ignored when comparing 2 struct tag ids.   2. When used in an URL path, should be encoded by url-encoding (AKA percent-encoding).  See [doc](https://aptos.dev/concepts/basics-accounts) for more details.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

