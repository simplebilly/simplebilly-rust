# \EmissionsApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_emission_entry_api**](EmissionsApi.md#create_emission_entry_api) | **POST** /api/v1/bookkeeping/emissions/entries | 
[**create_emission_target_api**](EmissionsApi.md#create_emission_target_api) | **POST** /api/v1/bookkeeping/emissions/targets | 
[**delete_emission_entry_api**](EmissionsApi.md#delete_emission_entry_api) | **DELETE** /api/v1/bookkeeping/emissions/entries/{id} | 
[**delete_emission_target_api**](EmissionsApi.md#delete_emission_target_api) | **DELETE** /api/v1/bookkeeping/emissions/targets/{id} | 
[**emissions_entries_api**](EmissionsApi.md#emissions_entries_api) | **GET** /api/v1/bookkeeping/emissions/entries | 
[**emissions_export_api**](EmissionsApi.md#emissions_export_api) | **GET** /api/v1/bookkeeping/emissions/export | 
[**emissions_factors_api**](EmissionsApi.md#emissions_factors_api) | **GET** /api/v1/bookkeeping/emissions/factors | 
[**emissions_report_api**](EmissionsApi.md#emissions_report_api) | **GET** /api/v1/bookkeeping/emissions/report | 
[**emissions_targets_api**](EmissionsApi.md#emissions_targets_api) | **GET** /api/v1/bookkeeping/emissions/targets | 



## create_emission_entry_api

> models::EmissionEntry create_emission_entry_api(create_emission_entry)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_emission_entry** | [**CreateEmissionEntry**](CreateEmissionEntry.md) |  | [required] |

### Return type

[**models::EmissionEntry**](EmissionEntry.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_emission_target_api

> models::EmissionTarget create_emission_target_api(create_emission_target)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_emission_target** | [**CreateEmissionTarget**](CreateEmissionTarget.md) |  | [required] |

### Return type

[**models::EmissionTarget**](EmissionTarget.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_emission_entry_api

> delete_emission_entry_api(id)


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


## delete_emission_target_api

> delete_emission_target_api(id)


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


## emissions_entries_api

> Vec<models::EmissionEntry> emissions_entries_api(year)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | **i32** |  | [required] |

### Return type

[**Vec<models::EmissionEntry>**](EmissionEntry.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## emissions_export_api

> models::EmissionsExportResponse emissions_export_api(year)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | **i32** |  | [required] |

### Return type

[**models::EmissionsExportResponse**](EmissionsExportResponse.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## emissions_factors_api

> Vec<models::EmissionFactorResponse> emissions_factors_api()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::EmissionFactorResponse>**](EmissionFactorResponse.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## emissions_report_api

> models::EmissionsReport emissions_report_api(year)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | **i32** |  | [required] |

### Return type

[**models::EmissionsReport**](EmissionsReport.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## emissions_targets_api

> Vec<models::EmissionTarget> emissions_targets_api()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::EmissionTarget>**](EmissionTarget.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

