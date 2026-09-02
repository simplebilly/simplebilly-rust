# \ReportsApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bilanz_report_api**](ReportsApi.md#bilanz_report_api) | **GET** /api/v1/bookkeeping/reports/bilanz | Bilanz (Balance Sheet)
[**guv_report_api**](ReportsApi.md#guv_report_api) | **GET** /api/v1/bookkeeping/reports/guv | Gewinn- und Verlustrechnung (P&L statement)
[**kontenansicht_report_api**](ReportsApi.md#kontenansicht_report_api) | **GET** /api/v1/bookkeeping/reports/kontenansicht | Kontenansicht (Account Overview)
[**umsatzsteuer_report_api**](ReportsApi.md#umsatzsteuer_report_api) | **GET** /api/v1/bookkeeping/reports/umsatzsteuer | Umsatzsteuer-Voranmeldung (VAT report)



## bilanz_report_api

> models::BilanzReport bilanz_report_api(year, month, date_from, date_to, page, page_size)
Bilanz (Balance Sheet)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | Option<**i32**> |  |  |
**month** | Option<**i32**> |  |  |
**date_from** | Option<**String**> |  |  |
**date_to** | Option<**String**> |  |  |
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |

### Return type

[**models::BilanzReport**](BilanzReport.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## guv_report_api

> models::GuVReport guv_report_api(year, month, date_from, date_to, page, page_size)
Gewinn- und Verlustrechnung (P&L statement)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | Option<**i32**> |  |  |
**month** | Option<**i32**> |  |  |
**date_from** | Option<**String**> |  |  |
**date_to** | Option<**String**> |  |  |
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |

### Return type

[**models::GuVReport**](GuVReport.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kontenansicht_report_api

> models::KontoReport kontenansicht_report_api(year, month, date_from, date_to, page, page_size)
Kontenansicht (Account Overview)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | Option<**i32**> |  |  |
**month** | Option<**i32**> |  |  |
**date_from** | Option<**String**> |  |  |
**date_to** | Option<**String**> |  |  |
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |

### Return type

[**models::KontoReport**](KontoReport.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## umsatzsteuer_report_api

> models::UmsatzsteuerReport umsatzsteuer_report_api(year, month, date_from, date_to, page, page_size)
Umsatzsteuer-Voranmeldung (VAT report)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | Option<**i32**> |  |  |
**month** | Option<**i32**> |  |  |
**date_from** | Option<**String**> |  |  |
**date_to** | Option<**String**> |  |  |
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |

### Return type

[**models::UmsatzsteuerReport**](UmsatzsteuerReport.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

