# \LeadApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_leads_api**](LeadApi.md#list_leads_api) | **GET** /api/v1/support/leads | 
[**update_lead_api**](LeadApi.md#update_lead_api) | **PUT** /api/v1/support/leads/{lead_id} | 



## list_leads_api

> Vec<models::Lead> list_leads_api(status, source, search, page, page_size)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**status** | Option<**String**> |  |  |
**source** | Option<**String**> |  |  |
**search** | Option<**String**> |  |  |
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |

### Return type

[**Vec<models::Lead>**](Lead.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_lead_api

> models::Lead update_lead_api(lead_id, lead_update)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lead_id** | **uuid::Uuid** |  | [required] |
**lead_update** | [**LeadUpdate**](LeadUpdate.md) |  | [required] |

### Return type

[**models::Lead**](Lead.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

