# \GewinnverwendungApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**gewinnverwendung_api**](GewinnverwendungApi.md#gewinnverwendung_api) | **GET** /api/v1/bookkeeping/gewinnverwendung | 
[**gewinnverwendung_export_api**](GewinnverwendungApi.md#gewinnverwendung_export_api) | **GET** /api/v1/bookkeeping/gewinnverwendung/export | 



## gewinnverwendung_api

> models::GewinnverwendungsReport gewinnverwendung_api(year)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | **i32** |  | [required] |

### Return type

[**models::GewinnverwendungsReport**](GewinnverwendungsReport.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gewinnverwendung_export_api

> models::GewinnverwendungsExportResponse gewinnverwendung_export_api(year)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | **i32** |  | [required] |

### Return type

[**models::GewinnverwendungsExportResponse**](GewinnverwendungsExportResponse.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

