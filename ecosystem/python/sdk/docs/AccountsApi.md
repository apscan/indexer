# aptos_sdk.AccountsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_account**](AccountsApi.md#get_account) | **GET** /accounts/{address} | Get account
[**get_account_module**](AccountsApi.md#get_account_module) | **GET** /accounts/{address}/module/{module_name} | Get specific account module
[**get_account_modules**](AccountsApi.md#get_account_modules) | **GET** /accounts/{address}/modules | Get account modules
[**get_account_resource**](AccountsApi.md#get_account_resource) | **GET** /accounts/{address}/resource/{resource_type} | Get specific account resource
[**get_account_resources**](AccountsApi.md#get_account_resources) | **GET** /accounts/{address}/resources | Get account resources

# **get_account**
> AccountData get_account(address)

Get account

Return high level information about an account such as its sequence number.

### Example

```python
import aptos_sdk
from aptos_sdk.api import accounts_api
from aptos_sdk.model.account_data import AccountData
from aptos_sdk.model.aptos_error import AptosError
from pprint import pprint
# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = aptos_sdk.Configuration(
    host = "http://localhost"
)

# Enter a context with an instance of the API client
with aptos_sdk.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = accounts_api.AccountsApi(api_client)

    # example passing only required values which don't have defaults set
    path_params = {
        'address': "61959483996478237799081788855878961092127547100767458402256961684167818091681",
    }
    query_params = {
    }
    try:
        # Get account
        api_response = api_instance.get_account(
            path_params=path_params,
            query_params=query_params,
        )
        pprint(api_response)
    except aptos_sdk.ApiException as e:
        print("Exception when calling AccountsApi->get_account: %s\n" % e)

    # example passing only optional values
    path_params = {
        'address': "61959483996478237799081788855878961092127547100767458402256961684167818091681",
    }
    query_params = {
        'ledger_version': "32425224034",
    }
    try:
        # Get account
        api_response = api_instance.get_account(
            path_params=path_params,
            query_params=query_params,
        )
        pprint(api_response)
    except aptos_sdk.ApiException as e:
        print("Exception when calling AccountsApi->get_account: %s\n" % e)
```
### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
query_params | RequestQueryParams | |
path_params | RequestPathParams | |
accept_content_types | typing.Tuple[str] | default is ('application/json', 'application/x-bcs', ) | Tells the server the content type(s) that are accepted by the client
stream | bool | default is False | if True then the response.content will be streamed and loaded from a file like object. When downloading a file, set this to True to force the code to deserialize the content to a FileSchema file
timeout | typing.Optional[typing.Union[int, typing.Tuple]] | default is None | the timeout used by the rest client
skip_deserialization | bool | default is False | when True, headers and body will be unset and an instance of api_client.ApiResponseWithoutDeserialization will be returned

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
address | AddressSchema | | 

#### AddressSchema

Hex encoded 32 byte Aptos account address

Type | Description | Notes
------------- | ------------- | -------------
**str** | Hex encoded 32 byte Aptos account address | 

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
[**AccountData**](AccountData.md) |  | 


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



[**AccountData**](AccountData.md)

### Authorization

No authorization required

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_account_module**
> MoveModuleBytecode get_account_module(addressmodule_name)

Get specific account module

This endpoint returns the module with a specific name residing at a given account at a specified ledger version (AKA transaction version). If the ledger version is not specified in the request, the latest ledger version is used.  The Aptos nodes prune account state history, via a configurable time window (link). If the requested data has been pruned, the server responds with a 404.

### Example

```python
import aptos_sdk
from aptos_sdk.api import accounts_api
from aptos_sdk.model.aptos_error import AptosError
from aptos_sdk.model.move_module_bytecode import MoveModuleBytecode
from pprint import pprint
# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = aptos_sdk.Configuration(
    host = "http://localhost"
)

# Enter a context with an instance of the API client
with aptos_sdk.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = accounts_api.AccountsApi(api_client)

    # example passing only required values which don't have defaults set
    path_params = {
        'address': "61959483996478237799081788855878961092127547100767458402256961684167818091681",
        'module_name': "module_name_example",
    }
    query_params = {
    }
    try:
        # Get specific account module
        api_response = api_instance.get_account_module(
            path_params=path_params,
            query_params=query_params,
        )
        pprint(api_response)
    except aptos_sdk.ApiException as e:
        print("Exception when calling AccountsApi->get_account_module: %s\n" % e)

    # example passing only optional values
    path_params = {
        'address': "61959483996478237799081788855878961092127547100767458402256961684167818091681",
        'module_name': "module_name_example",
    }
    query_params = {
        'ledger_version': "32425224034",
    }
    try:
        # Get specific account module
        api_response = api_instance.get_account_module(
            path_params=path_params,
            query_params=query_params,
        )
        pprint(api_response)
    except aptos_sdk.ApiException as e:
        print("Exception when calling AccountsApi->get_account_module: %s\n" % e)
```
### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
query_params | RequestQueryParams | |
path_params | RequestPathParams | |
accept_content_types | typing.Tuple[str] | default is ('application/json', 'application/x-bcs', ) | Tells the server the content type(s) that are accepted by the client
stream | bool | default is False | if True then the response.content will be streamed and loaded from a file like object. When downloading a file, set this to True to force the code to deserialize the content to a FileSchema file
timeout | typing.Optional[typing.Union[int, typing.Tuple]] | default is None | the timeout used by the rest client
skip_deserialization | bool | default is False | when True, headers and body will be unset and an instance of api_client.ApiResponseWithoutDeserialization will be returned

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
address | AddressSchema | | 
module_name | ModuleNameSchema | | 

#### AddressSchema

Hex encoded 32 byte Aptos account address

Type | Description | Notes
------------- | ------------- | -------------
**str** | Hex encoded 32 byte Aptos account address | 

#### ModuleNameSchema

Type | Description | Notes
------------- | ------------- | -------------
**str** |  | 

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
[**MoveModuleBytecode**](MoveModuleBytecode.md) |  | 


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



[**MoveModuleBytecode**](MoveModuleBytecode.md)

### Authorization

No authorization required

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_account_modules**
> [MoveModuleBytecode] get_account_modules(address)

Get account modules

This endpoint returns all account modules at a given address at a specific ledger version (AKA transaction version). If the ledger version is not specified in the request, the latest ledger version is used.  The Aptos nodes prune account state history, via a configurable time window (link). If the requested data has been pruned, the server responds with a 404.

### Example

```python
import aptos_sdk
from aptos_sdk.api import accounts_api
from aptos_sdk.model.aptos_error import AptosError
from aptos_sdk.model.move_module_bytecode import MoveModuleBytecode
from pprint import pprint
# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = aptos_sdk.Configuration(
    host = "http://localhost"
)

# Enter a context with an instance of the API client
with aptos_sdk.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = accounts_api.AccountsApi(api_client)

    # example passing only required values which don't have defaults set
    path_params = {
        'address': "61959483996478237799081788855878961092127547100767458402256961684167818091681",
    }
    query_params = {
    }
    try:
        # Get account modules
        api_response = api_instance.get_account_modules(
            path_params=path_params,
            query_params=query_params,
        )
        pprint(api_response)
    except aptos_sdk.ApiException as e:
        print("Exception when calling AccountsApi->get_account_modules: %s\n" % e)

    # example passing only optional values
    path_params = {
        'address': "61959483996478237799081788855878961092127547100767458402256961684167818091681",
    }
    query_params = {
        'ledger_version': "32425224034",
    }
    try:
        # Get account modules
        api_response = api_instance.get_account_modules(
            path_params=path_params,
            query_params=query_params,
        )
        pprint(api_response)
    except aptos_sdk.ApiException as e:
        print("Exception when calling AccountsApi->get_account_modules: %s\n" % e)
```
### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
query_params | RequestQueryParams | |
path_params | RequestPathParams | |
accept_content_types | typing.Tuple[str] | default is ('application/json', 'application/x-bcs', ) | Tells the server the content type(s) that are accepted by the client
stream | bool | default is False | if True then the response.content will be streamed and loaded from a file like object. When downloading a file, set this to True to force the code to deserialize the content to a FileSchema file
timeout | typing.Optional[typing.Union[int, typing.Tuple]] | default is None | the timeout used by the rest client
skip_deserialization | bool | default is False | when True, headers and body will be unset and an instance of api_client.ApiResponseWithoutDeserialization will be returned

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
address | AddressSchema | | 

#### AddressSchema

Hex encoded 32 byte Aptos account address

Type | Description | Notes
------------- | ------------- | -------------
**str** | Hex encoded 32 byte Aptos account address | 

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

Type | Description | Notes
------------- | ------------- | -------------
**[MoveModuleBytecode]** |  | 

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



[**[MoveModuleBytecode]**](MoveModuleBytecode.md)

### Authorization

No authorization required

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_account_resource**
> MoveResource get_account_resource(addressresource_type)

Get specific account resource

This endpoint returns the resource of a specific type residing at a given account at a specified ledger version (AKA transaction version). If the ledger version is not specified in the request, the latest ledger version is used.  The Aptos nodes prune account state history, via a configurable time window (link). If the requested data has been pruned, the server responds with a 404.

### Example

```python
import aptos_sdk
from aptos_sdk.api import accounts_api
from aptos_sdk.model.move_resource import MoveResource
from aptos_sdk.model.aptos_error import AptosError
from aptos_sdk.model.move_struct_tag_param import MoveStructTagParam
from pprint import pprint
# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = aptos_sdk.Configuration(
    host = "http://localhost"
)

# Enter a context with an instance of the API client
with aptos_sdk.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = accounts_api.AccountsApi(api_client)

    # example passing only required values which don't have defaults set
    path_params = {
        'address': "61959483996478237799081788855878961092127547100767458402256961684167818091681",
        'resource_type': MoveStructTagParam("0x1::coin::CoinStore<0x1::aptos_coin::AptosCoin>"),
    }
    query_params = {
    }
    try:
        # Get specific account resource
        api_response = api_instance.get_account_resource(
            path_params=path_params,
            query_params=query_params,
        )
        pprint(api_response)
    except aptos_sdk.ApiException as e:
        print("Exception when calling AccountsApi->get_account_resource: %s\n" % e)

    # example passing only optional values
    path_params = {
        'address': "61959483996478237799081788855878961092127547100767458402256961684167818091681",
        'resource_type': MoveStructTagParam("0x1::coin::CoinStore<0x1::aptos_coin::AptosCoin>"),
    }
    query_params = {
        'ledger_version': "32425224034",
    }
    try:
        # Get specific account resource
        api_response = api_instance.get_account_resource(
            path_params=path_params,
            query_params=query_params,
        )
        pprint(api_response)
    except aptos_sdk.ApiException as e:
        print("Exception when calling AccountsApi->get_account_resource: %s\n" % e)
```
### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
query_params | RequestQueryParams | |
path_params | RequestPathParams | |
accept_content_types | typing.Tuple[str] | default is ('application/json', 'application/x-bcs', ) | Tells the server the content type(s) that are accepted by the client
stream | bool | default is False | if True then the response.content will be streamed and loaded from a file like object. When downloading a file, set this to True to force the code to deserialize the content to a FileSchema file
timeout | typing.Optional[typing.Union[int, typing.Tuple]] | default is None | the timeout used by the rest client
skip_deserialization | bool | default is False | when True, headers and body will be unset and an instance of api_client.ApiResponseWithoutDeserialization will be returned

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
address | AddressSchema | | 
resource_type | ResourceTypeSchema | | 

#### AddressSchema

Hex encoded 32 byte Aptos account address

Type | Description | Notes
------------- | ------------- | -------------
**str** | Hex encoded 32 byte Aptos account address | 

#### ResourceTypeSchema
Type | Description  | Notes
------------- | ------------- | -------------
[**MoveStructTagParam**](MoveStructTagParam.md) |  | 


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
[**MoveResource**](MoveResource.md) |  | 


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



[**MoveResource**](MoveResource.md)

### Authorization

No authorization required

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_account_resources**
> [MoveResource] get_account_resources(address)

Get account resources

This endpoint returns all account resources at a given address at a specific ledger version (AKA transaction version). If the ledger version is not specified in the request, the latest ledger version is used.  The Aptos nodes prune account state history, via a configurable time window (link). If the requested data has been pruned, the server responds with a 404.

### Example

```python
import aptos_sdk
from aptos_sdk.api import accounts_api
from aptos_sdk.model.move_resource import MoveResource
from aptos_sdk.model.aptos_error import AptosError
from pprint import pprint
# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = aptos_sdk.Configuration(
    host = "http://localhost"
)

# Enter a context with an instance of the API client
with aptos_sdk.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = accounts_api.AccountsApi(api_client)

    # example passing only required values which don't have defaults set
    path_params = {
        'address': "61959483996478237799081788855878961092127547100767458402256961684167818091681",
    }
    query_params = {
    }
    try:
        # Get account resources
        api_response = api_instance.get_account_resources(
            path_params=path_params,
            query_params=query_params,
        )
        pprint(api_response)
    except aptos_sdk.ApiException as e:
        print("Exception when calling AccountsApi->get_account_resources: %s\n" % e)

    # example passing only optional values
    path_params = {
        'address': "61959483996478237799081788855878961092127547100767458402256961684167818091681",
    }
    query_params = {
        'ledger_version': "32425224034",
    }
    try:
        # Get account resources
        api_response = api_instance.get_account_resources(
            path_params=path_params,
            query_params=query_params,
        )
        pprint(api_response)
    except aptos_sdk.ApiException as e:
        print("Exception when calling AccountsApi->get_account_resources: %s\n" % e)
```
### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
query_params | RequestQueryParams | |
path_params | RequestPathParams | |
accept_content_types | typing.Tuple[str] | default is ('application/json', 'application/x-bcs', ) | Tells the server the content type(s) that are accepted by the client
stream | bool | default is False | if True then the response.content will be streamed and loaded from a file like object. When downloading a file, set this to True to force the code to deserialize the content to a FileSchema file
timeout | typing.Optional[typing.Union[int, typing.Tuple]] | default is None | the timeout used by the rest client
skip_deserialization | bool | default is False | when True, headers and body will be unset and an instance of api_client.ApiResponseWithoutDeserialization will be returned

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
address | AddressSchema | | 

#### AddressSchema

Hex encoded 32 byte Aptos account address

Type | Description | Notes
------------- | ------------- | -------------
**str** | Hex encoded 32 byte Aptos account address | 

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

Type | Description | Notes
------------- | ------------- | -------------
**[MoveResource]** |  | 

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



[**[MoveResource]**](MoveResource.md)

### Authorization

No authorization required

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

