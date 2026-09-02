# \EmailTemplateApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_email_template**](EmailTemplateApi.md#create_email_template) | **POST** /api/v1/email-templates | 
[**delete_email_template**](EmailTemplateApi.md#delete_email_template) | **DELETE** /api/v1/email-templates/{email_template_id} | 
[**get_email_template**](EmailTemplateApi.md#get_email_template) | **GET** /api/v1/email-templates/{email_template_id} | 
[**list_email_templates**](EmailTemplateApi.md#list_email_templates) | **GET** /api/v1/email-templates/ | 
[**render_email_template**](EmailTemplateApi.md#render_email_template) | **POST** /api/v1/email-templates/{email_template_id}/render | 
[**update_email_template**](EmailTemplateApi.md#update_email_template) | **PUT** /api/v1/email-templates/{email_template_id} | 



## create_email_template

> models::EmailTemplate create_email_template(email_template_create)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_template_create** | [**EmailTemplateCreate**](EmailTemplateCreate.md) |  | [required] |

### Return type

[**models::EmailTemplate**](EmailTemplate.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_email_template

> delete_email_template(email_template_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_template_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_email_template

> models::EmailTemplate get_email_template(email_template_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_template_id** | **String** |  | [required] |

### Return type

[**models::EmailTemplate**](EmailTemplate.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_email_templates

> Vec<models::EmailTemplate> list_email_templates(page, page_size, status, search)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**status** | Option<**String**> |  |  |
**search** | Option<**String**> |  |  |

### Return type

[**Vec<models::EmailTemplate>**](EmailTemplate.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## render_email_template

> serde_json::Value render_email_template(email_template_id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_template_id** | **String** |  | [required] |
**body** | Option<**serde_json::Value**> |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_email_template

> models::EmailTemplate update_email_template(email_template_id, email_template_update)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_template_id** | **String** |  | [required] |
**email_template_update** | [**EmailTemplateUpdate**](EmailTemplateUpdate.md) |  | [required] |

### Return type

[**models::EmailTemplate**](EmailTemplate.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

