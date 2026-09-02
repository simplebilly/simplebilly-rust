# \GewerbesteuerApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**gewerbesteuer_api**](GewerbesteuerApi.md#gewerbesteuer_api) | **GET** /api/v1/bookkeeping/gewerbesteuer | 



## gewerbesteuer_api

> models::GewerbesteuerErgebnis gewerbesteuer_api(year, hebesatz, gewerbeertrag, country, gemeindeschluessel)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | **i32** |  | [required] |
**hebesatz** | Option<**String**> |  |  |
**gewerbeertrag** | Option<**String**> |  |  |
**country** | Option<**String**> |  |  |
**gemeindeschluessel** | Option<**String**> |  |  |

### Return type

[**models::GewerbesteuerErgebnis**](GewerbesteuerErgebnis.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

