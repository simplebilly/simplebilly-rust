# \ComplianceTrainingApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_compliance_training**](ComplianceTrainingApi.md#create_compliance_training) | **POST** /api/v1/compliance-trainings | 
[**delete_compliance_training**](ComplianceTrainingApi.md#delete_compliance_training) | **DELETE** /api/v1/compliance-trainings/{id} | 
[**get_compliance_training**](ComplianceTrainingApi.md#get_compliance_training) | **GET** /api/v1/compliance-trainings/{id} | 
[**get_compliance_trainings**](ComplianceTrainingApi.md#get_compliance_trainings) | **GET** /api/v1/compliance-trainings/ | 
[**update_compliance_training**](ComplianceTrainingApi.md#update_compliance_training) | **PUT** /api/v1/compliance-trainings/{id} | 



## create_compliance_training

> models::ComplianceTraining create_compliance_training(compliance_training_create)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**compliance_training_create** | [**ComplianceTrainingCreate**](ComplianceTrainingCreate.md) |  | [required] |

### Return type

[**models::ComplianceTraining**](ComplianceTraining.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_compliance_training

> delete_compliance_training(id)


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


## get_compliance_training

> models::ComplianceTraining get_compliance_training(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::ComplianceTraining**](ComplianceTraining.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_compliance_trainings

> Vec<models::ComplianceTraining> get_compliance_trainings(page, page_size, search, include_deleted)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**search** | Option<**String**> |  |  |
**include_deleted** | Option<**bool**> | Soft-delete entities: set true to include rows with `deleted_at` set. |  |

### Return type

[**Vec<models::ComplianceTraining>**](ComplianceTraining.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_compliance_training

> models::ComplianceTraining update_compliance_training(id, compliance_training_update)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**compliance_training_update** | [**ComplianceTrainingUpdate**](ComplianceTrainingUpdate.md) |  | [required] |

### Return type

[**models::ComplianceTraining**](ComplianceTraining.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

