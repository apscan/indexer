# aptos_sdk.TablesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_table_item**](TablesApi.md#get_table_item) | **POST** /tables/{table_handle}/item | Get table item

# **get_table_item**
> MoveValue get_table_item(table_handletable_item_request)

Get table item

Get a table item from the table identified by {table_handle} in the path and the \"key\" (TableItemRequest) provided in the request body.  This is a POST endpoint because the \"key\" for requesting a specific table item (TableItemRequest) could be quite complex, as each of its fields could themselves be composed of other structs. This makes it impractical to express using query params, meaning GET isn't an option.

### Example

```python
import aptos_sdk
from aptos_sdk.api import tables_api
from aptos_sdk.model.aptos_error import AptosError
from aptos_sdk.model.table_item_request import TableItemRequest
from aptos_sdk.model.move_value import MoveValue
from pprint import pprint
# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = aptos_sdk.Configuration(
    host = "http://localhost"
)

# Enter a context with an instance of the API client
with aptos_sdk.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = tables_api.TablesApi(api_client)

    # example passing only required values which don't have defaults set
    path_params = {
        'table_handle': "340282366920938463463374607431768211454",
    }
    query_params = {
    }
    body = TableItemRequest(
        key_type=MoveType("signer"),
        value_type=MoveType("signer"),
        key=None,
    )
    try:
        # Get table item
        api_response = api_instance.get_table_item(
            path_params=path_params,
            query_params=query_params,
            body=body,
        )
        pprint(api_response)
    except aptos_sdk.ApiException as e:
        print("Exception when calling TablesApi->get_table_item: %s\n" % e)

    # example passing only optional values
    path_params = {
        'table_handle': "340282366920938463463374607431768211454",
    }
    query_params = {
        'ledger_version': "32425224034",
    }
    body = TableItemRequest(
        key_type=MoveType("signer"),
        value_type=MoveType("signer"),
        key=None,
    )
    try:
        # Get table item
        api_response = api_instance.get_table_item(
            path_params=path_params,
            query_params=query_params,
            body=body,
        )
        pprint(api_response)
    except aptos_sdk.ApiException as e:
        print("Exception when calling TablesApi->get_table_item: %s\n" % e)
```
### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
body | typing.Union[SchemaForRequestBodyApplicationJson] | required |
query_params | RequestQueryParams | |
path_params | RequestPathParams | |
content_type | str | optional, default is 'application/json' | Selects the schema and serialization of the request body
accept_content_types | typing.Tuple[str] | default is ('application/json', 'application/x-bcs', ) | Tells the server the content type(s) that are accepted by the client
stream | bool | default is False | if True then the response.content will be streamed and loaded from a file like object. When downloading a file, set this to True to force the code to deserialize the content to a FileSchema file
timeout | typing.Optional[typing.Union[int, typing.Tuple]] | default is None | the timeout used by the rest client
skip_deserialization | bool | default is False | when True, headers and body will be unset and an instance of api_client.ApiResponseWithoutDeserialization will be returned

### body

#### SchemaForRequestBodyApplicationJson
Type | Description  | Notes
------------- | ------------- | -------------
[**TableItemRequest**](TableItemRequest.md) |  | 


### query_params
#### RequestQueryParams

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
ledger_version | LedgerVersionSchema | | optional


#### LedgerVersionSchema

A string containing a 64-bit unsigned integer.  We represent u64 values as a string to ensure compatability with languages such as JavaScript that do not parse u64s in JSON natively. 

Type | Description | Notes
------------- | ------------- | -------------
**str** | A string containing a 64-bit unsigned integer.  We represent u64 values as a string to ensure compatability with languages such as JavaScript that do not parse u64s in JSON natively.  | 

### path_params
#### RequestPathParams

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
table_handle | TableHandleSchema | | 

#### TableHandleSchema

A string containing a 128-bit unsigned integer.  We represent u128 values as a string to ensure compatability with languages such as JavaScript that do not parse u64s in JSON natively. 

Type | Description | Notes
------------- | ------------- | -------------
**str** | A string containing a 128-bit unsigned integer.  We represent u128 values as a string to ensure compatability with languages such as JavaScript that do not parse u64s in JSON natively.  | 

### Return Types, Responses

Code | Class | Description
------------- | ------------- | -------------
n/a | api_client.ApiResponseWithoutDeserialization | When skip_deserialization is True this response is returned
200 | ApiResponseFor200 | 
400 | ApiResponseFor400 | 
404 | ApiResponseFor404 | 
500 | ApiResponseFor500 | 

#### ApiResponseFor200
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
response | urllib3.HTTPResponse | Raw response |
body | typing.Union[SchemaFor200ResponseBodyApplicationJson, SchemaFor200ResponseBodyApplicationXBcs, ] |  |
headers | ResponseHeadersFor200 |  |

#### SchemaFor200ResponseBodyApplicationJson
Type | Description  | Notes
------------- | ------------- | -------------
[**MoveValue**](MoveValue.md) |  | 


#### SchemaFor200ResponseBodyApplicationXBcs

Type | Description | Notes
------------- | ------------- | -------------
**[int]** |  | 
#### ResponseHeadersFor200

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
X-APTOS-CHAIN-ID | XAPTOSCHAINIDSchema | | 
X-APTOS-LEDGER-VERSION | XAPTOSLEDGERVERSIONSchema | | 
X-APTOS-LEDGER-OLDEST-VERSION | XAPTOSLEDGEROLDESTVERSIONSchema | | 
X-APTOS-LEDGER-TIMESTAMPUSEC | XAPTOSLEDGERTIMESTAMPUSECSchema | | 
X-APTOS-EPOCH | XAPTOSEPOCHSchema | | 

#### XAPTOSCHAINIDSchema

Type | Description | Notes
------------- | ------------- | -------------
**int** |  | 

#### XAPTOSLEDGERVERSIONSchema

Type | Description | Notes
------------- | ------------- | -------------
**int** |  | 

#### XAPTOSLEDGEROLDESTVERSIONSchema

Type | Description | Notes
------------- | ------------- | -------------
**int** |  | 

#### XAPTOSLEDGERTIMESTAMPUSECSchema

Type | Description | Notes
------------- | ------------- | -------------
**int** |  | 

#### XAPTOSEPOCHSchema

Type | Description | Notes
------------- | ------------- | -------------
**int** |  | 


#### ApiResponseFor400
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
response | urllib3.HTTPResponse | Raw response |
body | typing.Union[SchemaFor400ResponseBodyApplicationJson, ] |  |
headers | Unset | headers were not defined |

#### SchemaFor400ResponseBodyApplicationJson
Type | Description  | Notes
------------- | ------------- | -------------
[**AptosError**](AptosError.md) |  | 


#### ApiResponseFor404
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
response | urllib3.HTTPResponse | Raw response |
body | typing.Union[SchemaFor404ResponseBodyApplicationJson, ] |  |
headers | Unset | headers were not defined |

#### SchemaFor404ResponseBodyApplicationJson
Type | Description  | Notes
------------- | ------------- | -------------
[**AptosError**](AptosError.md) |  | 


#### ApiResponseFor500
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
response | urllib3.HTTPResponse | Raw response |
body | typing.Union[SchemaFor500ResponseBodyApplicationJson, ] |  |
headers | Unset | headers were not defined |

#### SchemaFor500ResponseBodyApplicationJson
Type | Description  | Notes
------------- | ------------- | -------------
[**AptosError**](AptosError.md) |  | 



[**MoveValue**](MoveValue.md)

### Authorization

No authorization required

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

