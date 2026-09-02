# \OrderConfirmationApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_confirmation**](OrderConfirmationApi.md#create_confirmation) | **POST** /api/v1/order-confirmations | 
[**delete_confirmation**](OrderConfirmationApi.md#delete_confirmation) | **DELETE** /api/v1/order-confirmations/{confirmation_id} | 
[**download_confirmation_pdf**](OrderConfirmationApi.md#download_confirmation_pdf) | **GET** /api/v1/order-confirmations/{confirmation_id}/pdf | 
[**get_confirmation**](OrderConfirmationApi.md#get_confirmation) | **GET** /api/v1/order-confirmations/{confirmation_id} | 
[**list_confirmations**](OrderConfirmationApi.md#list_confirmations) | **GET** /api/v1/order-confirmations/ | 
[**orderconfirmation_restore**](OrderConfirmationApi.md#orderconfirmation_restore) | **POST** /api/v1/order-confirmations/{confirmation_id}/restore | 
[**pursue_confirmation**](OrderConfirmationApi.md#pursue_confirmation) | **POST** /api/v1/order-confirmations/{confirmation_id}/pursue | 



## create_confirmation

> models::OrderConfirmation create_confirmation(order_confirmation_create)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_confirmation_create** | [**OrderConfirmationCreate**](OrderConfirmationCreate.md) |  | [required] |

### Return type

[**models::OrderConfirmation**](OrderConfirmation.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_confirmation

> delete_confirmation(confirmation_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**confirmation_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## download_confirmation_pdf

> download_confirmation_pdf(confirmation_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**confirmation_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/pdf, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_confirmation

> models::OrderConfirmation get_confirmation(confirmation_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**confirmation_id** | **String** |  | [required] |

### Return type

[**models::OrderConfirmation**](OrderConfirmation.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_confirmations

> Vec<models::OrderConfirmation> list_confirmations(page, page_size, search, include_deleted)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**search** | Option<**String**> |  |  |
**include_deleted** | Option<**bool**> | Soft-delete entities: set true to include rows with `deleted_at` set. |  |

### Return type

[**Vec<models::OrderConfirmation>**](OrderConfirmation.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orderconfirmation_restore

> models::OrderConfirmation orderconfirmation_restore(confirmation_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**confirmation_id** | **String** |  | [required] |

### Return type

[**models::OrderConfirmation**](OrderConfirmation.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pursue_confirmation

> models::DeliveryNote pursue_confirmation(confirmation_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**confirmation_id** | **String** |  | [required] |

### Return type

[**models::DeliveryNote**](DeliveryNote.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

