# \ProductionOrderApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_production_order**](ProductionOrderApi.md#create_production_order) | **POST** /api/v1/production-orders | 
[**delete_production_order**](ProductionOrderApi.md#delete_production_order) | **DELETE** /api/v1/production-orders/{production_order_id} | 
[**get_production_order**](ProductionOrderApi.md#get_production_order) | **GET** /api/v1/production-orders/{production_order_id} | 
[**list_production_orders**](ProductionOrderApi.md#list_production_orders) | **GET** /api/v1/production-orders/ | 
[**production_order_costing**](ProductionOrderApi.md#production_order_costing) | **GET** /api/v1/production-orders/{production_order_id}/costing | Actual-costing report (Nachkalkulation) — material costs from BOM components at their purchase price plus the resulting per-unit cost and margin against the finished product's sale price.
[**update_production_order**](ProductionOrderApi.md#update_production_order) | **PUT** /api/v1/production-orders/{production_order_id} | 
[**update_production_order_status**](ProductionOrderApi.md#update_production_order_status) | **PUT** /api/v1/production-orders/{production_order_id}/status | 



## create_production_order

> models::ProductionOrder create_production_order(production_order)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**production_order** | [**ProductionOrder**](ProductionOrder.md) |  | [required] |

### Return type

[**models::ProductionOrder**](ProductionOrder.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_production_order

> delete_production_order(production_order_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**production_order_id** | **uuid::Uuid** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_production_order

> models::ProductionOrder get_production_order(production_order_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**production_order_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::ProductionOrder**](ProductionOrder.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_production_orders

> Vec<models::ProductionOrder> list_production_orders(page, page_size, search, status)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**search** | Option<**String**> |  |  |
**status** | Option<**String**> | Filter by status. |  |

### Return type

[**Vec<models::ProductionOrder>**](ProductionOrder.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## production_order_costing

> models::ProductionOrderCosting production_order_costing(production_order_id)
Actual-costing report (Nachkalkulation) — material costs from BOM components at their purchase price plus the resulting per-unit cost and margin against the finished product's sale price.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**production_order_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::ProductionOrderCosting**](ProductionOrderCosting.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_production_order

> models::ProductionOrder update_production_order(production_order_id, production_order)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**production_order_id** | **uuid::Uuid** |  | [required] |
**production_order** | [**ProductionOrder**](ProductionOrder.md) |  | [required] |

### Return type

[**models::ProductionOrder**](ProductionOrder.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_production_order_status

> models::ProductionOrder update_production_order_status(production_order_id, production_order_status_update)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**production_order_id** | **uuid::Uuid** |  | [required] |
**production_order_status_update** | [**ProductionOrderStatusUpdate**](ProductionOrderStatusUpdate.md) |  | [required] |

### Return type

[**models::ProductionOrder**](ProductionOrder.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

