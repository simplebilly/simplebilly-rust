# \PurchaseOrderApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_purchase_order**](PurchaseOrderApi.md#create_purchase_order) | **POST** /api/v1/purchase-orders | 
[**delete_purchase_order**](PurchaseOrderApi.md#delete_purchase_order) | **DELETE** /api/v1/purchase-orders/{purchase_order_id} | 
[**get_purchase_order**](PurchaseOrderApi.md#get_purchase_order) | **GET** /api/v1/purchase-orders/{purchase_order_id} | 
[**list_purchase_orders**](PurchaseOrderApi.md#list_purchase_orders) | **GET** /api/v1/purchase-orders/ | 
[**match_invoice**](PurchaseOrderApi.md#match_invoice) | **POST** /api/v1/purchase-orders/{purchase_order_id}/match-invoice | 3-way invoice check (Rechnungsprüfung): compares the purchase order line items, the quantities received via goods receipts, and the supplier invoice line items, reporting quantity and price variances per product.
[**update_purchase_order**](PurchaseOrderApi.md#update_purchase_order) | **PUT** /api/v1/purchase-orders/{purchase_order_id} | 
[**update_purchase_order_status**](PurchaseOrderApi.md#update_purchase_order_status) | **PUT** /api/v1/purchase-orders/{purchase_order_id}/status | 



## create_purchase_order

> models::PurchaseOrder create_purchase_order(purchase_order)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**purchase_order** | [**PurchaseOrder**](PurchaseOrder.md) |  | [required] |

### Return type

[**models::PurchaseOrder**](PurchaseOrder.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_purchase_order

> delete_purchase_order(purchase_order_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**purchase_order_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_purchase_order

> models::PurchaseOrder get_purchase_order(purchase_order_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**purchase_order_id** | **String** |  | [required] |

### Return type

[**models::PurchaseOrder**](PurchaseOrder.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_purchase_orders

> Vec<models::PurchaseOrder> list_purchase_orders(page, page_size, status, supplier_name, search)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**status** | Option<**String**> |  |  |
**supplier_name** | Option<**String**> |  |  |
**search** | Option<**String**> |  |  |

### Return type

[**Vec<models::PurchaseOrder>**](PurchaseOrder.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## match_invoice

> serde_json::Value match_invoice(purchase_order_id, invoice_match_request)
3-way invoice check (Rechnungsprüfung): compares the purchase order line items, the quantities received via goods receipts, and the supplier invoice line items, reporting quantity and price variances per product.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**purchase_order_id** | **String** |  | [required] |
**invoice_match_request** | [**InvoiceMatchRequest**](InvoiceMatchRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_purchase_order

> models::PurchaseOrder update_purchase_order(purchase_order_id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**purchase_order_id** | **String** |  | [required] |
**body** | Option<**serde_json::Value**> |  | [required] |

### Return type

[**models::PurchaseOrder**](PurchaseOrder.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_purchase_order_status

> models::PurchaseOrder update_purchase_order_status(purchase_order_id, purchase_order_status_update)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**purchase_order_id** | **String** |  | [required] |
**purchase_order_status_update** | [**PurchaseOrderStatusUpdate**](PurchaseOrderStatusUpdate.md) |  | [required] |

### Return type

[**models::PurchaseOrder**](PurchaseOrder.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

