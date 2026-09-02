# \GezApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**gez_api**](GezApi.md#gez_api) | **GET** /api/v1/bookkeeping/gez | 



## gez_api

> models::GezReport gez_api(jahr, betriebsstaetten, kfz, hotelzimmer, beschaefigte)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**jahr** | Option<**i32**> |  |  |
**betriebsstaetten** | Option<**String**> | Liste der Betriebsstätten als JSON, z.B. `[{\"name\":\"Filiale 1\",\"beschaefigte\":12}]`. |  |
**kfz** | Option<**i64**> | Gesamtzahl der betrieblich genutzten Kfz (falls keine Betriebsstätten angegeben sind). |  |
**hotelzimmer** | Option<**i64**> | Gesamtzahl der Hotel-/Gästezimmer und Ferienwohnungen. |  |
**beschaefigte** | Option<**i64**> | Gesamtzahl der Beschäftigten (verwendet nur, wenn `betriebsstaetten` fehlt; dann wird eine einzelne Betriebsstätte angenommen). |  |

### Return type

[**models::GezReport**](GezReport.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

