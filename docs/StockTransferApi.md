# \StockTransferApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_stock_transfer**](StockTransferApi.md#create_stock_transfer) | **POST** /api/v1/stock-transfers | 
[**delete_stock_transfer**](StockTransferApi.md#delete_stock_transfer) | **DELETE** /api/v1/stock-transfers/{stock_transfer_id} | 
[**get_stock_transfer**](StockTransferApi.md#get_stock_transfer) | **GET** /api/v1/stock-transfers/{stock_transfer_id} | 
[**list_stock_transfers**](StockTransferApi.md#list_stock_transfers) | **GET** /api/v1/stock-transfers/ | 
[**update_stock_transfer_status**](StockTransferApi.md#update_stock_transfer_status) | **PUT** /api/v1/stock-transfers/{stock_transfer_id}/status | 



## create_stock_transfer

> models::StockTransfer create_stock_transfer(stock_transfer)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stock_transfer** | [**StockTransfer**](StockTransfer.md) |  | [required] |

### Return type

[**models::StockTransfer**](StockTransfer.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_stock_transfer

> delete_stock_transfer(stock_transfer_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stock_transfer_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_stock_transfer

> models::StockTransfer get_stock_transfer(stock_transfer_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stock_transfer_id** | **String** |  | [required] |

### Return type

[**models::StockTransfer**](StockTransfer.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_stock_transfers

> Vec<models::StockTransfer> list_stock_transfers(page, page_size, status, warehouse_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**status** | Option<**String**> |  |  |
**warehouse_id** | Option<**String**> |  |  |

### Return type

[**Vec<models::StockTransfer>**](StockTransfer.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_stock_transfer_status

> models::StockTransfer update_stock_transfer_status(stock_transfer_id, stock_transfer_status_update)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stock_transfer_id** | **String** |  | [required] |
**stock_transfer_status_update** | [**StockTransferStatusUpdate**](StockTransferStatusUpdate.md) |  | [required] |

### Return type

[**models::StockTransfer**](StockTransfer.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

