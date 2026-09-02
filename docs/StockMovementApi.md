# \StockMovementApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_stock_movement**](StockMovementApi.md#get_stock_movement) | **GET** /api/v1/stock-movements/{movement_id} | 
[**list_stock_movements**](StockMovementApi.md#list_stock_movements) | **GET** /api/v1/stock-movements/ | 



## get_stock_movement

> models::StockMovement get_stock_movement(movement_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**movement_id** | **String** |  | [required] |

### Return type

[**models::StockMovement**](StockMovement.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_stock_movements

> Vec<models::StockMovement> list_stock_movements(page, page_size, product_id, warehouse_id, movement_type, from, to)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**product_id** | Option<**uuid::Uuid**> |  |  |
**warehouse_id** | Option<**String**> |  |  |
**movement_type** | Option<**String**> |  |  |
**from** | Option<**chrono::NaiveDate**> | Only movements on or after this date (inclusive). |  |
**to** | Option<**chrono::NaiveDate**> | Only movements on or before this date (inclusive). |  |

### Return type

[**Vec<models::StockMovement>**](StockMovement.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

