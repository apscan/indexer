# aptos_sdk.TransactionsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**encode_submission**](TransactionsApi.md#encode_submission) | **POST** /transactions/encode_submission | Encode submission
[**get_account_transactions**](TransactionsApi.md#get_account_transactions) | **GET** /accounts/{address}/transactions | Get account transactions
[**get_transaction_by_hash**](TransactionsApi.md#get_transaction_by_hash) | **GET** /transactions/by_hash/{txn_hash} | Get transaction by hash
[**get_transaction_by_version**](TransactionsApi.md#get_transaction_by_version) | **GET** /transactions/by_version/{txn_version} | Get transaction by version
[**get_transactions**](TransactionsApi.md#get_transactions) | **GET** /transactions | Get transactions
[**simulate_transaction**](TransactionsApi.md#simulate_transaction) | **POST** /transactions/simulate | Simulate transaction
[**submit_transaction**](TransactionsApi.md#submit_transaction) | **POST** /transactions | Submit transaction

# **encode_submission**
> str encode_submission(encode_submission_request)

Encode submission

This endpoint accepts an EncodeSubmissionRequest, which internally is a UserTransactionRequestInner (and optionally secondary signers) encoded as JSON, validates the request format, and then returns that request encoded in BCS. The client can then use this to create a transaction signature to be used in a SubmitTransactionRequest, which it then passes to the /transactions POST endpoint.  To be clear, this endpoint makes it possible to submit transaction requests to the API from languages that do not have library support for BCS. If you are using an SDK that has BCS support, such as the official Rust, TypeScript, or Python SDKs, you do not need to use this endpoint.  To sign a message using the response from this endpoint: - Decode the hex encoded string in the response to bytes. - Sign the bytes to create the signature. - Use that as the signature field in something like Ed25519Signature, which you then use to build a TransactionSignature.

### Example

```python
import aptos_sdk
from aptos_sdk.api import transactions_api
from aptos_sdk.model.aptos_error import AptosError
from aptos_sdk.model.encode_submission_request import EncodeSubmissionRequest
from pprint import pprint
# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = aptos_sdk.Configuration(
    host = "http://localhost"
)

# Enter a context with an instance of the API client
with aptos_sdk.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = transactions_api.TransactionsApi(api_client)

    # example passing only required values which don't have defaults set
    body = EncodeSubmissionRequest(
        sender="61959483996478237799081788855878961092127547100767458402256961684167818091681",
        sequence_number="32425224034",
        max_gas_amount="32425224034",
        gas_unit_price="32425224034",
        expiration_timestamp_secs="32425224034",
        payload=TransactionPayload(),
        secondary_signers=[
            "61959483996478237799081788855878961092127547100767458402256961684167818091681"
        ],
    )
    try:
        # Encode submission
        api_response = api_instance.encode_submission(
            body=body,
        )
        pprint(api_response)
    except aptos_sdk.ApiException as e:
        print("Exception when calling TransactionsApi->encode_submission: %s\n" % e)
```
### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
body | typing.Union[SchemaForRequestBodyApplicationJson] | required |
content_type | str | optional, default is 'application/json' | Selects the schema and serialization of the request body
accept_content_types | typing.Tuple[str] | default is ('application/json', 'application/x-bcs', ) | Tells the server the content type(s) that are accepted by the client
stream | bool | default is False | if True then the response.content will be streamed and loaded from a file like object. When downloading a file, set this to True to force the code to deserialize the content to a FileSchema file
timeout | typing.Optional[typing.Union[int, typing.Tuple]] | default is None | the timeout used by the rest client
skip_deserialization | bool | default is False | when True, headers and body will be unset and an instance of api_client.ApiResponseWithoutDeserialization will be returned

### body

#### SchemaForRequestBodyApplicationJson
Type | Description  | Notes
------------- | ------------- | -------------
[**EncodeSubmissionRequest**](EncodeSubmissionRequest.md) |  | 


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

All bytes (Vec<u8>) data is represented as hex-encoded string prefixed with `0x` and fulfilled with two hex digits per byte.  Unlike the `Address` type, HexEncodedBytes will not trim any zeros. 

Type | Description | Notes
------------- | ------------- | -------------
**str** | All bytes (Vec&lt;u8&gt;) data is represented as hex-encoded string prefixed with &#x60;0x&#x60; and fulfilled with two hex digits per byte.  Unlike the &#x60;Address&#x60; type, HexEncodedBytes will not trim any zeros.  | 

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



**str**

### Authorization

No authorization required

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_account_transactions**
> [Transaction] get_account_transactions(address)

Get account transactions

todo

### Example

```python
import aptos_sdk
from aptos_sdk.api import transactions_api
from aptos_sdk.model.aptos_error import AptosError
from aptos_sdk.model.transaction import Transaction
from pprint import pprint
# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = aptos_sdk.Configuration(
    host = "http://localhost"
)

# Enter a context with an instance of the API client
with aptos_sdk.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = transactions_api.TransactionsApi(api_client)

    # example passing only required values which don't have defaults set
    path_params = {
        'address': "61959483996478237799081788855878961092127547100767458402256961684167818091681",
    }
    query_params = {
    }
    try:
        # Get account transactions
        api_response = api_instance.get_account_transactions(
            path_params=path_params,
            query_params=query_params,
        )
        pprint(api_response)
    except aptos_sdk.ApiException as e:
        print("Exception when calling TransactionsApi->get_account_transactions: %s\n" % e)

    # example passing only optional values
    path_params = {
        'address': "61959483996478237799081788855878961092127547100767458402256961684167818091681",
    }
    query_params = {
        'start': "32425224034",
        'limit': 1,
    }
    try:
        # Get account transactions
        api_response = api_instance.get_account_transactions(
            path_params=path_params,
            query_params=query_params,
        )
        pprint(api_response)
    except aptos_sdk.ApiException as e:
        print("Exception when calling TransactionsApi->get_account_transactions: %s\n" % e)
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
**[Transaction]** |  | 

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



[**[Transaction]**](Transaction.md)

### Authorization

No authorization required

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_transaction_by_hash**
> Transaction get_transaction_by_hash(txn_hash)

Get transaction by hash

Look up a transaction by its hash. This is the same hash that is returned by the API when submitting a transaction (see PendingTransaction).  When given a transaction hash, the server first looks for the transaction in storage (on-chain, committed). If no on-chain transaction is found, it looks the transaction up by hash in the mempool (pending, not yet committed).  To create a transaction hash by yourself, do the following: 1. Hash message bytes: \"RawTransaction\" bytes + BCS bytes of [Transaction](https://aptos-labs.github.io/aptos-core/aptos_types/transaction/enum.Transaction.html). 2. Apply hash algorithm `SHA3-256` to the hash message bytes. 3. Hex-encode the hash bytes with `0x` prefix.

### Example

```python
import aptos_sdk
from aptos_sdk.api import transactions_api
from aptos_sdk.model.aptos_error import AptosError
from aptos_sdk.model.transaction import Transaction
from pprint import pprint
# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = aptos_sdk.Configuration(
    host = "http://localhost"
)

# Enter a context with an instance of the API client
with aptos_sdk.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = transactions_api.TransactionsApi(api_client)

    # example passing only required values which don't have defaults set
    path_params = {
        'txn_hash': "txn_hash_example",
    }
    try:
        # Get transaction by hash
        api_response = api_instance.get_transaction_by_hash(
            path_params=path_params,
        )
        pprint(api_response)
    except aptos_sdk.ApiException as e:
        print("Exception when calling TransactionsApi->get_transaction_by_hash: %s\n" % e)
```
### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
path_params | RequestPathParams | |
accept_content_types | typing.Tuple[str] | default is ('application/json', 'application/x-bcs', ) | Tells the server the content type(s) that are accepted by the client
stream | bool | default is False | if True then the response.content will be streamed and loaded from a file like object. When downloading a file, set this to True to force the code to deserialize the content to a FileSchema file
timeout | typing.Optional[typing.Union[int, typing.Tuple]] | default is None | the timeout used by the rest client
skip_deserialization | bool | default is False | when True, headers and body will be unset and an instance of api_client.ApiResponseWithoutDeserialization will be returned

### path_params
#### RequestPathParams

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
txn_hash | TxnHashSchema | | 

#### TxnHashSchema

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
[**Transaction**](Transaction.md) |  | 


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



[**Transaction**](Transaction.md)

### Authorization

No authorization required

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_transaction_by_version**
> Transaction get_transaction_by_version(txn_version)

Get transaction by version

todo

### Example

```python
import aptos_sdk
from aptos_sdk.api import transactions_api
from aptos_sdk.model.aptos_error import AptosError
from aptos_sdk.model.transaction import Transaction
from pprint import pprint
# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = aptos_sdk.Configuration(
    host = "http://localhost"
)

# Enter a context with an instance of the API client
with aptos_sdk.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = transactions_api.TransactionsApi(api_client)

    # example passing only required values which don't have defaults set
    path_params = {
        'txn_version': "32425224034",
    }
    try:
        # Get transaction by version
        api_response = api_instance.get_transaction_by_version(
            path_params=path_params,
        )
        pprint(api_response)
    except aptos_sdk.ApiException as e:
        print("Exception when calling TransactionsApi->get_transaction_by_version: %s\n" % e)
```
### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
path_params | RequestPathParams | |
accept_content_types | typing.Tuple[str] | default is ('application/json', 'application/x-bcs', ) | Tells the server the content type(s) that are accepted by the client
stream | bool | default is False | if True then the response.content will be streamed and loaded from a file like object. When downloading a file, set this to True to force the code to deserialize the content to a FileSchema file
timeout | typing.Optional[typing.Union[int, typing.Tuple]] | default is None | the timeout used by the rest client
skip_deserialization | bool | default is False | when True, headers and body will be unset and an instance of api_client.ApiResponseWithoutDeserialization will be returned

### path_params
#### RequestPathParams

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
txn_version | TxnVersionSchema | | 

#### TxnVersionSchema

A string containing a 64-bit unsigned integer.  We represent u64 values as a string to ensure compatability with languages such as JavaScript that do not parse u64s in JSON natively. 

Type | Description | Notes
------------- | ------------- | -------------
**str** | A string containing a 64-bit unsigned integer.  We represent u64 values as a string to ensure compatability with languages such as JavaScript that do not parse u64s in JSON natively.  | 

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
[**Transaction**](Transaction.md) |  | 


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



[**Transaction**](Transaction.md)

### Authorization

No authorization required

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_transactions**
> [Transaction] get_transactions()

Get transactions

Get on-chain (meaning, committed) transactions. You may specify from when you want the transactions and how to include in the response.

### Example

```python
import aptos_sdk
from aptos_sdk.api import transactions_api
from aptos_sdk.model.aptos_error import AptosError
from aptos_sdk.model.transaction import Transaction
from pprint import pprint
# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = aptos_sdk.Configuration(
    host = "http://localhost"
)

# Enter a context with an instance of the API client
with aptos_sdk.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = transactions_api.TransactionsApi(api_client)

    # example passing only optional values
    query_params = {
        'start': "32425224034",
        'limit': 1,
    }
    try:
        # Get transactions
        api_response = api_instance.get_transactions(
            query_params=query_params,
        )
        pprint(api_response)
    except aptos_sdk.ApiException as e:
        print("Exception when calling TransactionsApi->get_transactions: %s\n" % e)
```
### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
query_params | RequestQueryParams | |
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
**[Transaction]** |  | 

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



[**[Transaction]**](Transaction.md)

### Authorization

No authorization required

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **simulate_transaction**
> [UserTransaction] simulate_transaction(submit_transaction_request)

Simulate transaction

Simulate submitting a transaction. To use this, you must: - Create a SignedTransaction with a zero-padded signature. - Submit a SubmitTransactionRequest containing a UserTransactionRequest containing that signature.  To use this endpoint with BCS, you must submit a SignedTransaction encoded as BCS. See SignedTransaction in types/src/transaction/mod.rs.

### Example

```python
import aptos_sdk
from aptos_sdk.api import transactions_api
from aptos_sdk.model.aptos_error import AptosError
from aptos_sdk.model.submit_transaction_request import SubmitTransactionRequest
from aptos_sdk.model.user_transaction import UserTransaction
from pprint import pprint
# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = aptos_sdk.Configuration(
    host = "http://localhost"
)

# Enter a context with an instance of the API client
with aptos_sdk.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = transactions_api.TransactionsApi(api_client)

    # example passing only required values which don't have defaults set
    body = SubmitTransactionRequest(
        sender="61959483996478237799081788855878961092127547100767458402256961684167818091681",
        sequence_number="32425224034",
        max_gas_amount="32425224034",
        gas_unit_price="32425224034",
        expiration_timestamp_secs="32425224034",
        payload=TransactionPayload(),
        signature=TransactionSignature(),
    )
    try:
        # Simulate transaction
        api_response = api_instance.simulate_transaction(
            body=body,
        )
        pprint(api_response)
    except aptos_sdk.ApiException as e:
        print("Exception when calling TransactionsApi->simulate_transaction: %s\n" % e)
```
### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
body | typing.Union[SchemaForRequestBodyApplicationJson, SchemaForRequestBodyApplicationXAptosSignedTransactionbcs] | required |
content_type | str | optional, default is 'application/json' | Selects the schema and serialization of the request body
accept_content_types | typing.Tuple[str] | default is ('application/json', 'application/x-bcs', ) | Tells the server the content type(s) that are accepted by the client
stream | bool | default is False | if True then the response.content will be streamed and loaded from a file like object. When downloading a file, set this to True to force the code to deserialize the content to a FileSchema file
timeout | typing.Optional[typing.Union[int, typing.Tuple]] | default is None | the timeout used by the rest client
skip_deserialization | bool | default is False | when True, headers and body will be unset and an instance of api_client.ApiResponseWithoutDeserialization will be returned

### body

#### SchemaForRequestBodyApplicationJson
Type | Description  | Notes
------------- | ------------- | -------------
[**SubmitTransactionRequest**](SubmitTransactionRequest.md) |  | 


#### SchemaForRequestBodyApplicationXAptosSignedTransactionbcs

Type | Description | Notes
------------- | ------------- | -------------
**[int]** |  | 

### Return Types, Responses

Code | Class | Description
------------- | ------------- | -------------
n/a | api_client.ApiResponseWithoutDeserialization | When skip_deserialization is True this response is returned
200 | ApiResponseFor200 | 
400 | ApiResponseFor400 | 
413 | ApiResponseFor413 | 
500 | ApiResponseFor500 | 
507 | ApiResponseFor507 | 

#### ApiResponseFor200
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
response | urllib3.HTTPResponse | Raw response |
body | typing.Union[SchemaFor200ResponseBodyApplicationJson, SchemaFor200ResponseBodyApplicationXBcs, ] |  |
headers | ResponseHeadersFor200 |  |

#### SchemaFor200ResponseBodyApplicationJson

Type | Description | Notes
------------- | ------------- | -------------
**[UserTransaction]** |  | 

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


#### ApiResponseFor413
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
response | urllib3.HTTPResponse | Raw response |
body | typing.Union[SchemaFor413ResponseBodyApplicationJson, ] |  |
headers | Unset | headers were not defined |

#### SchemaFor413ResponseBodyApplicationJson
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


#### ApiResponseFor507
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
response | urllib3.HTTPResponse | Raw response |
body | typing.Union[SchemaFor507ResponseBodyApplicationJson, ] |  |
headers | Unset | headers were not defined |

#### SchemaFor507ResponseBodyApplicationJson
Type | Description  | Notes
------------- | ------------- | -------------
[**AptosError**](AptosError.md) |  | 



[**[UserTransaction]**](UserTransaction.md)

### Authorization

No authorization required

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **submit_transaction**
> PendingTransaction submit_transaction(submit_transaction_request)

Submit transaction

This endpoint accepts transaction submissions in two formats.  To submit a transaction as JSON, you must submit a SubmitTransactionRequest. To build this request, do the following:  1. Encode the transaction as BCS. If you are using a language that has native BCS support, make sure of that library. If not, you may take advantage of /transactions/encode_submission. When using this endpoint, make sure you trust the node you're talking to, as it is possible they could manipulate your request. 2. Sign the encoded transaction and use it to create a TransactionSignature. 3. Submit the request. Make sure to use the \"application/json\" Content-Type.  To submit a transaction as BCS, you must submit a SignedTransaction encoded as BCS. See SignedTransaction in types/src/transaction/mod.rs.

### Example

```python
import aptos_sdk
from aptos_sdk.api import transactions_api
from aptos_sdk.model.aptos_error import AptosError
from aptos_sdk.model.pending_transaction import PendingTransaction
from aptos_sdk.model.submit_transaction_request import SubmitTransactionRequest
from pprint import pprint
# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = aptos_sdk.Configuration(
    host = "http://localhost"
)

# Enter a context with an instance of the API client
with aptos_sdk.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = transactions_api.TransactionsApi(api_client)

    # example passing only required values which don't have defaults set
    body = SubmitTransactionRequest(
        sender="61959483996478237799081788855878961092127547100767458402256961684167818091681",
        sequence_number="32425224034",
        max_gas_amount="32425224034",
        gas_unit_price="32425224034",
        expiration_timestamp_secs="32425224034",
        payload=TransactionPayload(),
        signature=TransactionSignature(),
    )
    try:
        # Submit transaction
        api_response = api_instance.submit_transaction(
            body=body,
        )
        pprint(api_response)
    except aptos_sdk.ApiException as e:
        print("Exception when calling TransactionsApi->submit_transaction: %s\n" % e)
```
### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
body | typing.Union[SchemaForRequestBodyApplicationJson, SchemaForRequestBodyApplicationXAptosSignedTransactionbcs] | required |
content_type | str | optional, default is 'application/json' | Selects the schema and serialization of the request body
accept_content_types | typing.Tuple[str] | default is ('application/json', 'application/x-bcs', ) | Tells the server the content type(s) that are accepted by the client
stream | bool | default is False | if True then the response.content will be streamed and loaded from a file like object. When downloading a file, set this to True to force the code to deserialize the content to a FileSchema file
timeout | typing.Optional[typing.Union[int, typing.Tuple]] | default is None | the timeout used by the rest client
skip_deserialization | bool | default is False | when True, headers and body will be unset and an instance of api_client.ApiResponseWithoutDeserialization will be returned

### body

#### SchemaForRequestBodyApplicationJson
Type | Description  | Notes
------------- | ------------- | -------------
[**SubmitTransactionRequest**](SubmitTransactionRequest.md) |  | 


#### SchemaForRequestBodyApplicationXAptosSignedTransactionbcs

Type | Description | Notes
------------- | ------------- | -------------
**[int]** |  | 

### Return Types, Responses

Code | Class | Description
------------- | ------------- | -------------
n/a | api_client.ApiResponseWithoutDeserialization | When skip_deserialization is True this response is returned
202 | ApiResponseFor202 | 
400 | ApiResponseFor400 | 
413 | ApiResponseFor413 | 
500 | ApiResponseFor500 | 
507 | ApiResponseFor507 | 

#### ApiResponseFor202
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
response | urllib3.HTTPResponse | Raw response |
body | typing.Union[SchemaFor202ResponseBodyApplicationJson, SchemaFor202ResponseBodyApplicationXBcs, ] |  |
headers | ResponseHeadersFor202 |  |

#### SchemaFor202ResponseBodyApplicationJson
Type | Description  | Notes
------------- | ------------- | -------------
[**PendingTransaction**](PendingTransaction.md) |  | 


#### SchemaFor202ResponseBodyApplicationXBcs

Type | Description | Notes
------------- | ------------- | -------------
**[int]** |  | 
#### ResponseHeadersFor202

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


#### ApiResponseFor413
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
response | urllib3.HTTPResponse | Raw response |
body | typing.Union[SchemaFor413ResponseBodyApplicationJson, ] |  |
headers | Unset | headers were not defined |

#### SchemaFor413ResponseBodyApplicationJson
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


#### ApiResponseFor507
Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
response | urllib3.HTTPResponse | Raw response |
body | typing.Union[SchemaFor507ResponseBodyApplicationJson, ] |  |
headers | Unset | headers were not defined |

#### SchemaFor507ResponseBodyApplicationJson
Type | Description  | Notes
------------- | ------------- | -------------
[**AptosError**](AptosError.md) |  | 



[**PendingTransaction**](PendingTransaction.md)

### Authorization

No authorization required

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

