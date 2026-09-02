# \EbilanzApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ebilanz_report_api**](EbilanzApi.md#ebilanz_report_api) | **GET** /api/v1/bookkeeping/ebilanz | 
[**ebilanz_xbrl_export_api**](EbilanzApi.md#ebilanz_xbrl_export_api) | **GET** /api/v1/bookkeeping/ebilanz/xbrl | 



## ebilanz_report_api

> models::EBilanzReport ebilanz_report_api(year, date_from, date_to)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | Option<**i32**> |  |  |
**date_from** | Option<**String**> |  |  |
**date_to** | Option<**String**> |  |  |

### Return type

[**models::EBilanzReport**](EBilanzReport.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ebilanz_xbrl_export_api

> ebilanz_xbrl_export_api(year, date_from, date_to)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | Option<**i32**> |  |  |
**date_from** | Option<**String**> |  |  |
**date_to** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

