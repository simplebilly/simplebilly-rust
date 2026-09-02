# \AutomationsApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_automations**](AutomationsApi.md#list_automations) | **GET** /api/v1/automations | 
[**trigger_automation**](AutomationsApi.md#trigger_automation) | **POST** /api/v1/automations/{key}/trigger | 
[**update_automation**](AutomationsApi.md#update_automation) | **PUT** /api/v1/automations/{key} | 



## list_automations

> Vec<models::AutomationDto> list_automations()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::AutomationDto>**](AutomationDto.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trigger_automation

> serde_json::Value trigger_automation(key)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_automation

> models::AutomationDto update_automation(key, update_automation)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** |  | [required] |
**update_automation** | [**UpdateAutomation**](UpdateAutomation.md) |  | [required] |

### Return type

[**models::AutomationDto**](AutomationDto.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

