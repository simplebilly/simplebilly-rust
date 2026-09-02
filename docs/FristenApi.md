# \FristenApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**fristen_api**](FristenApi.md#fristen_api) | **GET** /api/v1/bookkeeping/fristen | 



## fristen_api

> models::FristenErgebnis fristen_api(bundesland, voranmeldungsrhythmus, dauerfristverlaengerung, est_aktiv, gewst_aktiv, monate)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bundesland** | Option<**String**> |  |  |
**voranmeldungsrhythmus** | Option<**String**> |  |  |
**dauerfristverlaengerung** | Option<**bool**> |  |  |
**est_aktiv** | Option<**bool**> |  |  |
**gewst_aktiv** | Option<**bool**> |  |  |
**monate** | Option<**i32**> |  |  |

### Return type

[**models::FristenErgebnis**](FristenErgebnis.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

