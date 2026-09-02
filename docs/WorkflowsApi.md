# \WorkflowsApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_workflows_api**](WorkflowsApi.md#list_workflows_api) | **GET** /api/v1/workflows | 
[**set_workflow_enabled_api**](WorkflowsApi.md#set_workflow_enabled_api) | **PUT** /api/v1/workflows/{workflow_id}/enabled | 



## list_workflows_api

> Vec<models::Workflow> list_workflows_api()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Workflow>**](Workflow.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_workflow_enabled_api

> models::Workflow set_workflow_enabled_api(workflow_id, workflow_enabled_update)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workflow_id** | **String** |  | [required] |
**workflow_enabled_update** | [**WorkflowEnabledUpdate**](WorkflowEnabledUpdate.md) |  | [required] |

### Return type

[**models::Workflow**](Workflow.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

