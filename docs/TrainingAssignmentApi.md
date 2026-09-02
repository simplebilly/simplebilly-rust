# \TrainingAssignmentApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_training_assignment**](TrainingAssignmentApi.md#create_training_assignment) | **POST** /api/v1/training-assignments | 
[**delete_training_assignment**](TrainingAssignmentApi.md#delete_training_assignment) | **DELETE** /api/v1/training-assignments/{id} | 
[**get_training_assignment**](TrainingAssignmentApi.md#get_training_assignment) | **GET** /api/v1/training-assignments/{id} | 
[**get_training_assignments**](TrainingAssignmentApi.md#get_training_assignments) | **GET** /api/v1/training-assignments/ | 
[**update_training_assignment**](TrainingAssignmentApi.md#update_training_assignment) | **PUT** /api/v1/training-assignments/{id} | 



## create_training_assignment

> models::TrainingAssignment create_training_assignment(training_assignment_create)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**training_assignment_create** | [**TrainingAssignmentCreate**](TrainingAssignmentCreate.md) |  | [required] |

### Return type

[**models::TrainingAssignment**](TrainingAssignment.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_training_assignment

> delete_training_assignment(id)


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


## get_training_assignment

> models::TrainingAssignment get_training_assignment(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::TrainingAssignment**](TrainingAssignment.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_training_assignments

> Vec<models::TrainingAssignment> get_training_assignments(page, page_size, search, include_deleted)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**search** | Option<**String**> |  |  |
**include_deleted** | Option<**bool**> | Soft-delete entities: set true to include rows with `deleted_at` set. |  |

### Return type

[**Vec<models::TrainingAssignment>**](TrainingAssignment.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_training_assignment

> models::TrainingAssignment update_training_assignment(id, training_assignment_update)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**training_assignment_update** | [**TrainingAssignmentUpdate**](TrainingAssignmentUpdate.md) |  | [required] |

### Return type

[**models::TrainingAssignment**](TrainingAssignment.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

