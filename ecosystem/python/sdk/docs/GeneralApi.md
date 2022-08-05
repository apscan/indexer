# aptos_sdk.GeneralApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_ledger_info**](GeneralApi.md#get_ledger_info) | **GET** / | Get ledger info
[**spec**](GeneralApi.md#spec) | **GET** /spec | Show OpenAPI explorer

# **get_ledger_info**
> IndexResponse get_ledger_info()

Get ledger info

Get the latest ledger information, including data such as chain ID, role type, ledger versions, epoch, etc.

### Example

```python
import aptos_sdk
from aptos_sdk.api import general_api
from aptos_sdk.model.aptos_error import AptosError
from aptos_sdk.model.index_response import IndexResponse
from pprint import pprint
# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = aptos_sdk.Configuration(
    host = "http://localhost"
)

# Enter a context with an instance of the API client
with aptos_sdk.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = general_api.GeneralApi(api_client)

    # example, this endpoint has no required or optional parameters
    try:
        # Get ledger info
        api_response = api_instance.get_ledger_info()
        pprint(api_response)
    except aptos_sdk.ApiException as e:
        print("Exception when calling GeneralApi->get_ledger_info: %s\n" % e)
```
### Parameters
This endpoint does not need any parameter.

### Return Types, Responses

Code | Class | Description
------------- | ------------- | -------------
n/a | api_client.ApiResponseWithoutDeserialization | When skip_deserialization is True this response is returned
200 | ApiResponseFor200 | 
400 | ApiResponseFor400 | 
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
[**IndexResponse**](IndexResponse.md) |  | 


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



[**IndexResponse**](IndexResponse.md)

### Authorization

No authorization required

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **spec**
> str spec()

Show OpenAPI explorer

Provides a UI that you can use to explore the API. You can also retrieve the API directly at `/spec.yaml` and `/spec.json`.

### Example

```python
import aptos_sdk
from aptos_sdk.api import general_api
from pprint import pprint
# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = aptos_sdk.Configuration(
    host = "http://localhost"
)

# Enter a context with an instance of the API client
with aptos_sdk.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = general_api.GeneralApi(api_client)

    # example, this endpoint has no required or optional parameters
    try:
        # Show OpenAPI explorer
        api_response = api_instance.spec()
        pprint(api_response)
    except aptos_sdk.ApiException as e:
        print("Exception when calling GeneralApi->spec: %s\n" % e)
```
### Parameters
This endpoint does not need any parameter.

### Return Types, Responses

Code | Class | Description
------------- | ------------- | -------------
n/a | api_client.ApiResponseWithoutDeserialization | When skip_deserialization is True this response is returned
200 | ApiResponseFor200 | 

#### ApiResponseFor200
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
response | urllib3.HTTPResponse | Raw response |
body | typing.Union[SchemaFor200ResponseBodyTextHtml, ] |  |
headers | Unset | headers were not defined |

#### SchemaFor200ResponseBodyTextHtml

Type | Description | Notes
------------- | ------------- | -------------
**str** |  | 


**str**

### Authorization

No authorization required

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

