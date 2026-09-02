# \PaygapApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**paygap_auskunft_api**](PaygapApi.md#paygap_auskunft_api) | **GET** /api/v1/bookkeeping/paygap/auskunft/{employee_id} | 
[**paygap_export_api**](PaygapApi.md#paygap_export_api) | **GET** /api/v1/bookkeeping/paygap/export | 
[**paygap_report_api**](PaygapApi.md#paygap_report_api) | **GET** /api/v1/bookkeeping/paygap/report | 



## paygap_auskunft_api

> models::PayGapInfoResponse paygap_auskunft_api(employee_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**employee_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::PayGapInfoResponse**](PayGapInfoResponse.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## paygap_export_api

> models::PayGapExportResponse paygap_export_api()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::PayGapExportResponse**](PayGapExportResponse.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## paygap_report_api

> models::PayGapReport paygap_report_api()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::PayGapReport**](PayGapReport.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

