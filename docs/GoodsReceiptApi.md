# \GoodsReceiptApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_goods_receipt**](GoodsReceiptApi.md#create_goods_receipt) | **POST** /api/v1/goods-receipts | 
[**delete_goods_receipt**](GoodsReceiptApi.md#delete_goods_receipt) | **DELETE** /api/v1/goods-receipts/{goods_receipt_id} | 
[**get_goods_receipt**](GoodsReceiptApi.md#get_goods_receipt) | **GET** /api/v1/goods-receipts/{goods_receipt_id} | 
[**list_goods_receipts**](GoodsReceiptApi.md#list_goods_receipts) | **GET** /api/v1/goods-receipts/ | 



## create_goods_receipt

> models::GoodsReceipt create_goods_receipt(goods_receipt)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**goods_receipt** | [**GoodsReceipt**](GoodsReceipt.md) |  | [required] |

### Return type

[**models::GoodsReceipt**](GoodsReceipt.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_goods_receipt

> delete_goods_receipt(goods_receipt_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**goods_receipt_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_goods_receipt

> models::GoodsReceipt get_goods_receipt(goods_receipt_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**goods_receipt_id** | **String** |  | [required] |

### Return type

[**models::GoodsReceipt**](GoodsReceipt.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_goods_receipts

> Vec<models::GoodsReceipt> list_goods_receipts(page, page_size, purchase_order_id, supplier_name, warehouse_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**purchase_order_id** | Option<**String**> |  |  |
**supplier_name** | Option<**String**> |  |  |
**warehouse_id** | Option<**String**> |  |  |

### Return type

[**Vec<models::GoodsReceipt>**](GoodsReceipt.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

