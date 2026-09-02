# \WarehouseApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_warehouse**](WarehouseApi.md#create_warehouse) | **POST** /api/v1/warehouses | 
[**delete_warehouse**](WarehouseApi.md#delete_warehouse) | **DELETE** /api/v1/warehouses/{warehouse_id} | 
[**get_warehouse**](WarehouseApi.md#get_warehouse) | **GET** /api/v1/warehouses/{warehouse_id} | 
[**list_warehouses**](WarehouseApi.md#list_warehouses) | **GET** /api/v1/warehouses/ | 
[**update_warehouse**](WarehouseApi.md#update_warehouse) | **PUT** /api/v1/warehouses/{warehouse_id} | 



## create_warehouse

> models::Warehouse create_warehouse(warehouse)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**warehouse** | [**Warehouse**](Warehouse.md) |  | [required] |

### Return type

[**models::Warehouse**](Warehouse.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_warehouse

> delete_warehouse(warehouse_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**warehouse_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_warehouse

> models::Warehouse get_warehouse(warehouse_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**warehouse_id** | **String** |  | [required] |

### Return type

[**models::Warehouse**](Warehouse.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_warehouses

> Vec<models::Warehouse> list_warehouses(page, page_size, search, is_active)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**search** | Option<**String**> |  |  |
**is_active** | Option<**bool**> |  |  |

### Return type

[**Vec<models::Warehouse>**](Warehouse.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_warehouse

> models::Warehouse update_warehouse(warehouse_id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**warehouse_id** | **String** |  | [required] |
**body** | Option<**serde_json::Value**> |  | [required] |

### Return type

[**models::Warehouse**](Warehouse.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

