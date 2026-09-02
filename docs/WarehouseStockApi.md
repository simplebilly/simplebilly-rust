# \WarehouseStockApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_warehouse_stock**](WarehouseStockApi.md#create_warehouse_stock) | **POST** /api/v1/warehouses/{warehouse_id}/stock | 
[**delete_warehouse_stock**](WarehouseStockApi.md#delete_warehouse_stock) | **DELETE** /api/v1/warehouses/{warehouse_id}/stock/{product_id} | 
[**list_warehouse_stock**](WarehouseStockApi.md#list_warehouse_stock) | **GET** /api/v1/warehouses/{warehouse_id}/stock | 
[**update_warehouse_stock**](WarehouseStockApi.md#update_warehouse_stock) | **PUT** /api/v1/warehouses/{warehouse_id}/stock/{product_id} | 



## create_warehouse_stock

> models::WarehouseStock create_warehouse_stock(warehouse_id, stock_adjustment)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**warehouse_id** | **String** |  | [required] |
**stock_adjustment** | [**StockAdjustment**](StockAdjustment.md) |  | [required] |

### Return type

[**models::WarehouseStock**](WarehouseStock.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_warehouse_stock

> delete_warehouse_stock(warehouse_id, product_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**warehouse_id** | **String** |  | [required] |
**product_id** | **uuid::Uuid** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_warehouse_stock

> Vec<models::WarehouseStock> list_warehouse_stock(warehouse_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**warehouse_id** | **String** |  | [required] |

### Return type

[**Vec<models::WarehouseStock>**](WarehouseStock.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_warehouse_stock

> models::WarehouseStock update_warehouse_stock(warehouse_id, product_id, stock_adjustment)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**warehouse_id** | **String** |  | [required] |
**product_id** | **uuid::Uuid** |  | [required] |
**stock_adjustment** | [**StockAdjustment**](StockAdjustment.md) |  | [required] |

### Return type

[**models::WarehouseStock**](WarehouseStock.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

