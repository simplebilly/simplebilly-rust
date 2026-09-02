# \AiApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ai_suggest_api**](AiApi.md#ai_suggest_api) | **POST** /api/v1/support/ai/suggest | 
[**create_worker_api**](AiApi.md#create_worker_api) | **POST** /api/v1/support/ai/workers | 
[**list_workers_api**](AiApi.md#list_workers_api) | **GET** /api/v1/support/ai/workers | 
[**run_worker_api**](AiApi.md#run_worker_api) | **POST** /api/v1/support/ai/workers/{worker_id}/run | 



## ai_suggest_api

> models::AiSuggestion ai_suggest_api(ai_suggestion_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ai_suggestion_request** | [**AiSuggestionRequest**](AiSuggestionRequest.md) |  | [required] |

### Return type

[**models::AiSuggestion**](AiSuggestion.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_worker_api

> models::AiWorkerConfig create_worker_api(ai_config_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ai_config_dto** | [**AiConfigDto**](AiConfigDto.md) |  | [required] |

### Return type

[**models::AiWorkerConfig**](AiWorkerConfig.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_workers_api

> Vec<models::AiWorkerConfig> list_workers_api()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::AiWorkerConfig>**](AiWorkerConfig.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## run_worker_api

> models::AiSuggestion run_worker_api(worker_id, ai_suggestion_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**worker_id** | **uuid::Uuid** |  | [required] |
**ai_suggestion_request** | [**AiSuggestionRequest**](AiSuggestionRequest.md) |  | [required] |

### Return type

[**models::AiSuggestion**](AiSuggestion.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

