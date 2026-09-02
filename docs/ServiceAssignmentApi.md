# \ServiceAssignmentApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_service_assignment**](ServiceAssignmentApi.md#create_service_assignment) | **POST** /api/v1/service-assignments | 
[**delete_service_assignment**](ServiceAssignmentApi.md#delete_service_assignment) | **DELETE** /api/v1/service-assignments/{id} | 
[**get_service_assignment**](ServiceAssignmentApi.md#get_service_assignment) | **GET** /api/v1/service-assignments/{id} | 
[**get_service_assignments**](ServiceAssignmentApi.md#get_service_assignments) | **GET** /api/v1/service-assignments/ | 
[**update_service_assignment**](ServiceAssignmentApi.md#update_service_assignment) | **PUT** /api/v1/service-assignments/{id} | 



## create_service_assignment

> models::ServiceAssignment create_service_assignment(service_assignment_create)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_assignment_create** | [**ServiceAssignmentCreate**](ServiceAssignmentCreate.md) |  | [required] |

### Return type

[**models::ServiceAssignment**](ServiceAssignment.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_service_assignment

> delete_service_assignment(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_service_assignment

> models::ServiceAssignment get_service_assignment(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::ServiceAssignment**](ServiceAssignment.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_service_assignments

> Vec<models::ServiceAssignment> get_service_assignments(page, page_size, search, include_deleted)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**search** | Option<**String**> |  |  |
**include_deleted** | Option<**bool**> | Soft-delete entities: set true to include rows with `deleted_at` set. |  |

### Return type

[**Vec<models::ServiceAssignment>**](ServiceAssignment.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_service_assignment

> models::ServiceAssignment update_service_assignment(id, service_assignment_update)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**service_assignment_update** | [**ServiceAssignmentUpdate**](ServiceAssignmentUpdate.md) |  | [required] |

### Return type

[**models::ServiceAssignment**](ServiceAssignment.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

