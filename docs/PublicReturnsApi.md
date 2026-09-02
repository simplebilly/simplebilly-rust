# \PublicReturnsApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_public_return_status**](PublicReturnsApi.md#get_public_return_status) | **GET** /api/v1/public/returns/status | Customer checks the status of a return (public, no auth). The return is only revealed when its linked order's email matches.
[**list_public_returns**](PublicReturnsApi.md#list_public_returns) | **GET** /api/v1/public/returns/list | List all returns for an order (public, no auth).
[**request_public_return**](PublicReturnsApi.md#request_public_return) | **POST** /api/v1/public/returns/request | Customer requests a return for an order (public, no auth).



## get_public_return_status

> models::PublicReturnStatusResponse get_public_return_status(email, return_number, return_order_id, order_number)
Customer checks the status of a return (public, no auth). The return is only revealed when its linked order's email matches.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** |  | [required] |
**return_number** | Option<**String**> | Either return_number or return_order_id must be provided. |  |
**return_order_id** | Option<**String**> |  |  |
**order_number** | Option<**String**> |  |  |

### Return type

[**models::PublicReturnStatusResponse**](PublicReturnStatusResponse.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_public_returns

> Vec<models::PublicReturnStatusResponse> list_public_returns(order_number, email)
List all returns for an order (public, no auth).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_number** | **String** |  | [required] |
**email** | **String** |  | [required] |

### Return type

[**Vec<models::PublicReturnStatusResponse>**](PublicReturnStatusResponse.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## request_public_return

> models::PublicReturnResponse request_public_return(public_return_request)
Customer requests a return for an order (public, no auth).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**public_return_request** | [**PublicReturnRequest**](PublicReturnRequest.md) |  | [required] |

### Return type

[**models::PublicReturnResponse**](PublicReturnResponse.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

