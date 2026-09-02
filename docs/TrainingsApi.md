# \TrainingsApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_my_trainings**](TrainingsApi.md#get_my_trainings) | **GET** /api/v1/trainings/me | 
[**get_training_content**](TrainingsApi.md#get_training_content) | **GET** /api/v1/trainings/content/{code} | 
[**get_training_overview**](TrainingsApi.md#get_training_overview) | **GET** /api/v1/trainings/overview | 
[**submit_training_result**](TrainingsApi.md#submit_training_result) | **POST** /api/v1/trainings/submit-result | 



## get_my_trainings

> Vec<models::MyTrainingItem> get_my_trainings()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::MyTrainingItem>**](MyTrainingItem.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_training_content

> models::TrainingContent get_training_content(code)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | Training code, e.g. data_privacy | [required] |

### Return type

[**models::TrainingContent**](TrainingContent.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_training_overview

> Vec<models::HrTrainingOverview> get_training_overview()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::HrTrainingOverview>**](HrTrainingOverview.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submit_training_result

> models::SubmitResultResponse submit_training_result(submit_result_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**submit_result_dto** | [**SubmitResultDto**](SubmitResultDto.md) |  | [required] |

### Return type

[**models::SubmitResultResponse**](SubmitResultResponse.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

