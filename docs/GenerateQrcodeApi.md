# \GenerateQrcodeApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**generate_qrcode_api**](GenerateQrcodeApi.md#generate_qrcode_api) | **GET** /api/v1/invoices/{id}/qrcode | 



## generate_qrcode_api

> models::QrCodeResponse generate_qrcode_api(iban, id, holder_name, bic, amount, reference, purpose)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**iban** | **String** |  | [required] |
**id** | **String** |  | [required] |
**holder_name** | Option<**String**> |  |  |
**bic** | Option<**String**> |  |  |
**amount** | Option<**String**> |  |  |
**reference** | Option<**String**> |  |  |
**purpose** | Option<**String**> |  |  |

### Return type

[**models::QrCodeResponse**](QRCodeResponse.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

