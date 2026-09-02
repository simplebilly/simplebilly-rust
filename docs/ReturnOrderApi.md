# \ReturnOrderApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_return_order**](ReturnOrderApi.md#create_return_order) | **POST** /api/v1/returns | 
[**delete_return_order**](ReturnOrderApi.md#delete_return_order) | **DELETE** /api/v1/returns/{return_order_id} | 
[**get_return_order**](ReturnOrderApi.md#get_return_order) | **GET** /api/v1/returns/{return_order_id} | 
[**list_return_orders**](ReturnOrderApi.md#list_return_orders) | **GET** /api/v1/returns/ | 
[**return_logistics_queue**](ReturnOrderApi.md#return_logistics_queue) | **GET** /api/v1/returns/logistics-queue | 
[**return_logistics_summary**](ReturnOrderApi.md#return_logistics_summary) | **GET** /api/v1/returns/logistics-summary | Returns-logistics aggregation for the dashboard: quantities received, restocked and scrapped per warehouse.
[**update_return_order**](ReturnOrderApi.md#update_return_order) | **PUT** /api/v1/returns/{return_order_id} | 
[**update_return_order_status**](ReturnOrderApi.md#update_return_order_status) | **PUT** /api/v1/returns/{return_order_id}/status | 



## create_return_order

> models::ReturnOrder create_return_order(return_order)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**return_order** | [**ReturnOrder**](ReturnOrder.md) |  | [required] |

### Return type

[**models::ReturnOrder**](ReturnOrder.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_return_order

> delete_return_order(return_order_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**return_order_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_return_order

> models::ReturnOrder get_return_order(return_order_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**return_order_id** | **String** |  | [required] |

### Return type

[**models::ReturnOrder**](ReturnOrder.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_return_orders

> Vec<models::ReturnOrder> list_return_orders(page, page_size, status, customer_name, order_number)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**status** | Option<**String**> |  |  |
**customer_name** | Option<**String**> |  |  |
**order_number** | Option<**String**> |  |  |

### Return type

[**Vec<models::ReturnOrder>**](ReturnOrder.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## return_logistics_queue

> Vec<models::ReturnLogisticsQueueItem> return_logistics_queue()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ReturnLogisticsQueueItem>**](ReturnLogisticsQueueItem.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## return_logistics_summary

> models::ReturnLogisticsSummary return_logistics_summary()
Returns-logistics aggregation for the dashboard: quantities received, restocked and scrapped per warehouse.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ReturnLogisticsSummary**](ReturnLogisticsSummary.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_return_order

> models::ReturnOrder update_return_order(return_order_id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**return_order_id** | **String** |  | [required] |
**body** | Option<**serde_json::Value**> |  | [required] |

### Return type

[**models::ReturnOrder**](ReturnOrder.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_return_order_status

> models::ReturnOrder update_return_order_status(return_order_id, return_order_status_update)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**return_order_id** | **String** |  | [required] |
**return_order_status_update** | [**ReturnOrderStatusUpdate**](ReturnOrderStatusUpdate.md) |  | [required] |

### Return type

[**models::ReturnOrder**](ReturnOrder.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

