# \DownPaymentInvoiceApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**download_down_payment_invoice_pdf**](DownPaymentInvoiceApi.md#download_down_payment_invoice_pdf) | **GET** /api/v1/down-payment-invoices/{id}/pdf | 
[**get_down_payment_invoice**](DownPaymentInvoiceApi.md#get_down_payment_invoice) | **GET** /api/v1/down-payment-invoices/{id} | 
[**list_down_payment_invoices**](DownPaymentInvoiceApi.md#list_down_payment_invoices) | **GET** /api/v1/down-payment-invoices/ | 



## download_down_payment_invoice_pdf

> download_down_payment_invoice_pdf(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_down_payment_invoice

> models::DownPaymentInvoice get_down_payment_invoice(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::DownPaymentInvoice**](DownPaymentInvoice.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_down_payment_invoices

> Vec<models::DownPaymentInvoice> list_down_payment_invoices(page, page_size, search, include_deleted)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**search** | Option<**String**> |  |  |
**include_deleted** | Option<**bool**> | Soft-delete entities: set true to include rows with `deleted_at` set. |  |

### Return type

[**Vec<models::DownPaymentInvoice>**](DownPaymentInvoice.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

