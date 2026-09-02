# \DeliveryDateApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_delivery_date**](DeliveryDateApi.md#create_delivery_date) | **POST** /api/v1/delivery-dates | 
[**delete_delivery_date**](DeliveryDateApi.md#delete_delivery_date) | **DELETE** /api/v1/delivery-dates/{delivery_date_id} | 
[**get_delivery_date**](DeliveryDateApi.md#get_delivery_date) | **GET** /api/v1/delivery-dates/{delivery_date_id} | 
[**get_delivery_performance**](DeliveryDateApi.md#get_delivery_performance) | **GET** /api/v1/delivery-dates/performance | On-time performance summary: how many promised delivery dates were met within a period.
[**list_delivery_dates**](DeliveryDateApi.md#list_delivery_dates) | **GET** /api/v1/delivery-dates/ | 
[**update_delivery_date**](DeliveryDateApi.md#update_delivery_date) | **PUT** /api/v1/delivery-dates/{delivery_date_id} | 
[**update_delivery_date_status**](DeliveryDateApi.md#update_delivery_date_status) | **PUT** /api/v1/delivery-dates/{delivery_date_id}/status | 



## create_delivery_date

> models::DeliveryDate create_delivery_date(delivery_date_create)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delivery_date_create** | [**DeliveryDateCreate**](DeliveryDateCreate.md) |  | [required] |

### Return type

[**models::DeliveryDate**](DeliveryDate.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_delivery_date

> delete_delivery_date(delivery_date_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delivery_date_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_delivery_date

> models::DeliveryDate get_delivery_date(delivery_date_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delivery_date_id** | **String** |  | [required] |

### Return type

[**models::DeliveryDate**](DeliveryDate.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_delivery_performance

> serde_json::Value get_delivery_performance(page, page_size, order_number, status, from, to)
On-time performance summary: how many promised delivery dates were met within a period.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**order_number** | Option<**String**> |  |  |
**status** | Option<**String**> |  |  |
**from** | Option<**chrono::NaiveDate**> | Only dates on or after this date. |  |
**to** | Option<**chrono::NaiveDate**> | Only dates on or before this date. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_delivery_dates

> Vec<models::DeliveryDate> list_delivery_dates(page, page_size, order_number, status, from, to)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**order_number** | Option<**String**> |  |  |
**status** | Option<**String**> |  |  |
**from** | Option<**chrono::NaiveDate**> | Only dates on or after this date. |  |
**to** | Option<**chrono::NaiveDate**> | Only dates on or before this date. |  |

### Return type

[**Vec<models::DeliveryDate>**](DeliveryDate.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_delivery_date

> models::DeliveryDate update_delivery_date(delivery_date_id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delivery_date_id** | **String** |  | [required] |
**body** | Option<**serde_json::Value**> |  | [required] |

### Return type

[**models::DeliveryDate**](DeliveryDate.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_delivery_date_status

> models::DeliveryDate update_delivery_date_status(delivery_date_id, delivery_date_status_update)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delivery_date_id** | **String** |  | [required] |
**delivery_date_status_update** | [**DeliveryDateStatusUpdate**](DeliveryDateStatusUpdate.md) |  | [required] |

### Return type

[**models::DeliveryDate**](DeliveryDate.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

