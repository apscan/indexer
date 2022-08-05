# aptos_sdk.EventsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_events_by_event_handle**](EventsApi.md#get_events_by_event_handle) | **GET** /accounts/{address}/events/{event_handle}/{field_name} | Get events by event handle
[**get_events_by_event_key**](EventsApi.md#get_events_by_event_key) | **GET** /events/{event_key} | Get events by event key

# **get_events_by_event_handle**
> [Event] get_events_by_event_handle(addressevent_handlefield_name)

Get events by event handle

This API extracts event key from the account resource identified by the `event_handle_struct` and `field_name`, then returns events identified by the event key.

### Example

```python
import aptos_sdk
from aptos_sdk.api import events_api
from aptos_sdk.model.aptos_error import AptosError
from aptos_sdk.model.event import Event
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
    api_instance = events_api.EventsApi(api_client)

    # example passing only required values which don't have defaults set
    path_params = {
        'address': "61959483996478237799081788855878961092127547100767458402256961684167818091681",
        'event_handle': MoveStructTagParam("0x1::coin::CoinStore<0x1::aptos_coin::AptosCoin>"),
        'field_name': "field_name_example",
    }
    query_params = {
    }
    try:
        # Get events by event handle
        api_response = api_instance.get_events_by_event_handle(
            path_params=path_params,
            query_params=query_params,
        )
        pprint(api_response)
    except aptos_sdk.ApiException as e:
        print("Exception when calling EventsApi->get_events_by_event_handle: %s\n" % e)

    # example passing only optional values
    path_params = {
        'address': "61959483996478237799081788855878961092127547100767458402256961684167818091681",
        'event_handle': MoveStructTagParam("0x1::coin::CoinStore<0x1::aptos_coin::AptosCoin>"),
        'field_name': "field_name_example",
    }
    query_params = {
        'start': "32425224034",
        'limit': 1,
    }
    try:
        # Get events by event handle
        api_response = api_instance.get_events_by_event_handle(
            path_params=path_params,
            query_params=query_params,
        )
        pprint(api_response)
    except aptos_sdk.ApiException as e:
        print("Exception when calling EventsApi->get_events_by_event_handle: %s\n" % e)
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
start | StartSchema | | optional
limit | LimitSchema | | optional


#### StartSchema

A string containing a 64-bit unsigned integer.  We represent u64 values as a string to ensure compatability with languages such as JavaScript that do not parse u64s in JSON natively. 

Type | Description | Notes
------------- | ------------- | -------------
**str** | A string containing a 64-bit unsigned integer.  We represent u64 values as a string to ensure compatability with languages such as JavaScript that do not parse u64s in JSON natively.  | 

#### LimitSchema

Type | Description | Notes
------------- | ------------- | -------------
**int** |  | 

### path_params
#### RequestPathParams

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
address | AddressSchema | | 
event_handle | EventHandleSchema | | 
field_name | FieldNameSchema | | 

#### AddressSchema

Hex encoded 32 byte Aptos account address

Type | Description | Notes
------------- | ------------- | -------------
**str** | Hex encoded 32 byte Aptos account address | 

#### EventHandleSchema
Type | Description  | Notes
------------- | ------------- | -------------
[**MoveStructTagParam**](MoveStructTagParam.md) |  | 


#### FieldNameSchema

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

Type | Description | Notes
------------- | ------------- | -------------
**[Event]** |  | 

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



[**[Event]**](Event.md)

### Authorization

No authorization required

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_events_by_event_key**
> [Event] get_events_by_event_key(event_key)

Get events by event key

This endpoint allows you to get a list of events of a specific type as identified by its event key, which is a globally unique ID.

### Example

```python
import aptos_sdk
from aptos_sdk.api import events_api
from aptos_sdk.model.aptos_error import AptosError
from aptos_sdk.model.event import Event
from pprint import pprint
# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = aptos_sdk.Configuration(
    host = "http://localhost"
)

# Enter a context with an instance of the API client
with aptos_sdk.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = events_api.EventsApi(api_client)

    # example passing only required values which don't have defaults set
    path_params = {
        'event_key': "61959483996478237799081788855878961092127547100767458402256961684167818091681",
    }
    query_params = {
    }
    try:
        # Get events by event key
        api_response = api_instance.get_events_by_event_key(
            path_params=path_params,
            query_params=query_params,
        )
        pprint(api_response)
    except aptos_sdk.ApiException as e:
        print("Exception when calling EventsApi->get_events_by_event_key: %s\n" % e)

    # example passing only optional values
    path_params = {
        'event_key': "61959483996478237799081788855878961092127547100767458402256961684167818091681",
    }
    query_params = {
        'start': "32425224034",
        'limit': 1,
    }
    try:
        # Get events by event key
        api_response = api_instance.get_events_by_event_key(
            path_params=path_params,
            query_params=query_params,
        )
        pprint(api_response)
    except aptos_sdk.ApiException as e:
        print("Exception when calling EventsApi->get_events_by_event_key: %s\n" % e)
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
start | StartSchema | | optional
limit | LimitSchema | | optional


#### StartSchema

A string containing a 64-bit unsigned integer.  We represent u64 values as a string to ensure compatability with languages such as JavaScript that do not parse u64s in JSON natively. 

Type | Description | Notes
------------- | ------------- | -------------
**str** | A string containing a 64-bit unsigned integer.  We represent u64 values as a string to ensure compatability with languages such as JavaScript that do not parse u64s in JSON natively.  | 

#### LimitSchema

Type | Description | Notes
------------- | ------------- | -------------
**int** |  | 

### path_params
#### RequestPathParams

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
event_key | EventKeySchema | | 

#### EventKeySchema

Event key is a global index for an event stream.  It is hex-encoded BCS bytes of `EventHandle` `guid` field value, which is a combination of a `uint64` creation number and account address (without trimming leading zeros).  For example, event key `0x000000000000000088fbd33f54e1126269769780feb24480428179f552e2313fbe571b72e62a1ca1` is combined by the following 2 parts:   1. `0000000000000000`: `uint64` representation of `0`.   2. `88fbd33f54e1126269769780feb24480428179f552e2313fbe571b72e62a1ca1`: 32 bytes of account address. 

Type | Description | Notes
------------- | ------------- | -------------
**str** | Event key is a global index for an event stream.  It is hex-encoded BCS bytes of &#x60;EventHandle&#x60; &#x60;guid&#x60; field value, which is a combination of a &#x60;uint64&#x60; creation number and account address (without trimming leading zeros).  For example, event key &#x60;0x000000000000000088fbd33f54e1126269769780feb24480428179f552e2313fbe571b72e62a1ca1&#x60; is combined by the following 2 parts:   1. &#x60;0000000000000000&#x60;: &#x60;uint64&#x60; representation of &#x60;0&#x60;.   2. &#x60;88fbd33f54e1126269769780feb24480428179f552e2313fbe571b72e62a1ca1&#x60;: 32 bytes of account address.  | 

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
**[Event]** |  | 

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



[**[Event]**](Event.md)

### Authorization

No authorization required

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

