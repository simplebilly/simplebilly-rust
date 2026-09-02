# \RecurringTemplateApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_recurring_template**](RecurringTemplateApi.md#create_recurring_template) | **POST** /api/v1/recurring-templates | 
[**delete_recurring_template**](RecurringTemplateApi.md#delete_recurring_template) | **DELETE** /api/v1/recurring-templates/{template_id} | 
[**get_recurring_template**](RecurringTemplateApi.md#get_recurring_template) | **GET** /api/v1/recurring-templates/{template_id} | 
[**list_recurring_templates**](RecurringTemplateApi.md#list_recurring_templates) | **GET** /api/v1/recurring-templates/ | 



## create_recurring_template

> models::RecurringTemplate create_recurring_template(body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<**serde_json::Value**> |  | [required] |

### Return type

[**models::RecurringTemplate**](RecurringTemplate.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_recurring_template

> delete_recurring_template(template_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_recurring_template

> models::RecurringTemplate get_recurring_template(template_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **String** |  | [required] |

### Return type

[**models::RecurringTemplate**](RecurringTemplate.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_recurring_templates

> Vec<models::RecurringTemplate> list_recurring_templates()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::RecurringTemplate>**](RecurringTemplate.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

