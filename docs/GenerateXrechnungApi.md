# \GenerateXrechnungApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**generate_xrechnung_api**](GenerateXrechnungApi.md#generate_xrechnung_api) | **GET** /api/v1/invoices/{id}/xrechnung | 



## generate_xrechnung_api

> models::XRechnungResponse generate_xrechnung_api(id, supplier_name, supplier_street, supplier_city, supplier_zip, supplier_country, supplier_vat_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**supplier_name** | Option<**String**> |  |  |
**supplier_street** | Option<**String**> |  |  |
**supplier_city** | Option<**String**> |  |  |
**supplier_zip** | Option<**String**> |  |  |
**supplier_country** | Option<**String**> |  |  |
**supplier_vat_id** | Option<**String**> |  |  |

### Return type

[**models::XRechnungResponse**](XRechnungResponse.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

