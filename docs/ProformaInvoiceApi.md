# \ProformaInvoiceApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**convert_proforma_to_invoice**](ProformaInvoiceApi.md#convert_proforma_to_invoice) | **POST** /api/v1/proforma-invoices/{proforma_id}/convert | 
[**create_proforma_invoice**](ProformaInvoiceApi.md#create_proforma_invoice) | **POST** /api/v1/proforma-invoices | 
[**delete_proforma_invoice**](ProformaInvoiceApi.md#delete_proforma_invoice) | **DELETE** /api/v1/proforma-invoices/{proforma_id} | 
[**get_proforma_invoice**](ProformaInvoiceApi.md#get_proforma_invoice) | **GET** /api/v1/proforma-invoices/{proforma_id} | 
[**list_proforma_invoices**](ProformaInvoiceApi.md#list_proforma_invoices) | **GET** /api/v1/proforma-invoices/ | 
[**update_proforma_invoice**](ProformaInvoiceApi.md#update_proforma_invoice) | **PUT** /api/v1/proforma-invoices/{proforma_id} | 



## convert_proforma_to_invoice

> models::ConvertResponse convert_proforma_to_invoice(proforma_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proforma_id** | **String** |  | [required] |

### Return type

[**models::ConvertResponse**](ConvertResponse.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_proforma_invoice

> models::ProformaInvoice create_proforma_invoice(proforma_invoice)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proforma_invoice** | [**ProformaInvoice**](ProformaInvoice.md) |  | [required] |

### Return type

[**models::ProformaInvoice**](ProformaInvoice.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_proforma_invoice

> delete_proforma_invoice(proforma_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proforma_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_proforma_invoice

> models::ProformaInvoice get_proforma_invoice(proforma_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proforma_id** | **String** |  | [required] |

### Return type

[**models::ProformaInvoice**](ProformaInvoice.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_proforma_invoices

> Vec<models::ProformaInvoice> list_proforma_invoices(page, page_size, status, customer_id, order_number)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**status** | Option<**String**> |  |  |
**customer_id** | Option<**String**> |  |  |
**order_number** | Option<**String**> |  |  |

### Return type

[**Vec<models::ProformaInvoice>**](ProformaInvoice.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_proforma_invoice

> models::ProformaInvoice update_proforma_invoice(proforma_id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proforma_id** | **String** |  | [required] |
**body** | Option<**serde_json::Value**> |  | [required] |

### Return type

[**models::ProformaInvoice**](ProformaInvoice.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

