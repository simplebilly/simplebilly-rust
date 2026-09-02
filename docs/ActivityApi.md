# \ActivityApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_activity**](ActivityApi.md#create_activity) | **POST** /api/v1/activities | 
[**delete_activity**](ActivityApi.md#delete_activity) | **DELETE** /api/v1/activities/{activity_id} | 
[**get_activity**](ActivityApi.md#get_activity) | **GET** /api/v1/activities/{activity_id} | 
[**list_activities**](ActivityApi.md#list_activities) | **GET** /api/v1/activities/ | 
[**update_activity**](ActivityApi.md#update_activity) | **PUT** /api/v1/activities/{activity_id} | 
[**update_activity_status**](ActivityApi.md#update_activity_status) | **PUT** /api/v1/activities/{activity_id}/status | 



## create_activity

> models::Activity create_activity(activity)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**activity** | [**Activity**](Activity.md) |  | [required] |

### Return type

[**models::Activity**](Activity.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_activity

> delete_activity(activity_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**activity_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_activity

> models::Activity get_activity(activity_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**activity_id** | **String** |  | [required] |

### Return type

[**models::Activity**](Activity.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_activities

> Vec<models::Activity> list_activities(page, page_size, contact_id, activity_type, status, assigned_to, overdue_only)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**contact_id** | Option<**String**> |  |  |
**activity_type** | Option<**String**> |  |  |
**status** | Option<**String**> |  |  |
**assigned_to** | Option<**String**> |  |  |
**overdue_only** | Option<**bool**> | Only show overdue follow-ups. |  |

### Return type

[**Vec<models::Activity>**](Activity.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_activity

> models::Activity update_activity(activity_id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**activity_id** | **String** |  | [required] |
**body** | Option<**serde_json::Value**> |  | [required] |

### Return type

[**models::Activity**](Activity.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_activity_status

> models::Activity update_activity_status(activity_id, activity_status_update)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**activity_id** | **String** |  | [required] |
**activity_status_update** | [**ActivityStatusUpdate**](ActivityStatusUpdate.md) |  | [required] |

### Return type

[**models::Activity**](Activity.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

