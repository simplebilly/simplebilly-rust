# \OnlineshopApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_smtp_config_api**](OnlineshopApi.md#get_smtp_config_api) | **GET** /api/v1/settings/smtp | 
[**save_smtp_config_api**](OnlineshopApi.md#save_smtp_config_api) | **PUT** /api/v1/settings/smtp | 



## get_smtp_config_api

> models::SmtpConfig get_smtp_config_api()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SmtpConfig**](SmtpConfig.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## save_smtp_config_api

> models::SmtpConfig save_smtp_config_api(smtp_config)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**smtp_config** | Option<[**SmtpConfig**](SmtpConfig.md)> |  |  |

### Return type

[**models::SmtpConfig**](SmtpConfig.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

