# \SupplierInvoiceApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_supplier_invoice**](SupplierInvoiceApi.md#create_supplier_invoice) | **POST** /api/v1/supplier-invoices | 
[**delete_supplier_invoice**](SupplierInvoiceApi.md#delete_supplier_invoice) | **DELETE** /api/v1/supplier-invoices/{supplier_invoice_id} | 
[**get_supplier_invoice**](SupplierInvoiceApi.md#get_supplier_invoice) | **GET** /api/v1/supplier-invoices/{supplier_invoice_id} | 
[**list_supplier_invoices**](SupplierInvoiceApi.md#list_supplier_invoices) | **GET** /api/v1/supplier-invoices/ | 
[**update_supplier_invoice**](SupplierInvoiceApi.md#update_supplier_invoice) | **PUT** /api/v1/supplier-invoices/{supplier_invoice_id} | 
[**update_supplier_invoice_status**](SupplierInvoiceApi.md#update_supplier_invoice_status) | **PUT** /api/v1/supplier-invoices/{supplier_invoice_id}/status | 



## create_supplier_invoice

> models::SupplierInvoice create_supplier_invoice(supplier_invoice)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**supplier_invoice** | [**SupplierInvoice**](SupplierInvoice.md) |  | [required] |

### Return type

[**models::SupplierInvoice**](SupplierInvoice.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_supplier_invoice

> delete_supplier_invoice(supplier_invoice_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**supplier_invoice_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_supplier_invoice

> models::SupplierInvoice get_supplier_invoice(supplier_invoice_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**supplier_invoice_id** | **String** |  | [required] |

### Return type

[**models::SupplierInvoice**](SupplierInvoice.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_supplier_invoices

> Vec<models::SupplierInvoice> list_supplier_invoices(page, page_size, status, purchase_order_id, supplier_name)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**status** | Option<**String**> |  |  |
**purchase_order_id** | Option<**String**> |  |  |
**supplier_name** | Option<**String**> |  |  |

### Return type

[**Vec<models::SupplierInvoice>**](SupplierInvoice.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_supplier_invoice

> models::SupplierInvoice update_supplier_invoice(supplier_invoice_id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**supplier_invoice_id** | **String** |  | [required] |
**body** | Option<**serde_json::Value**> |  | [required] |

### Return type

[**models::SupplierInvoice**](SupplierInvoice.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_supplier_invoice_status

> models::SupplierInvoice update_supplier_invoice_status(supplier_invoice_id, supplier_invoice_status_update)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**supplier_invoice_id** | **String** |  | [required] |
**supplier_invoice_status_update** | [**SupplierInvoiceStatusUpdate**](SupplierInvoiceStatusUpdate.md) |  | [required] |

### Return type

[**models::SupplierInvoice**](SupplierInvoice.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

