# \InstituteProfileApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_institute_profile**](InstituteProfileApi.md#get_institute_profile) | **GET** /api/v1/institute-profile | Current institute profile (created with defaults when missing).
[**update_institute_profile**](InstituteProfileApi.md#update_institute_profile) | **PUT** /api/v1/institute-profile | Update the institute profile (institute_type and/or kapitalmarktorientiert).



## get_institute_profile

> models::InstituteProfile get_institute_profile()
Current institute profile (created with defaults when missing).

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::InstituteProfile**](InstituteProfile.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_institute_profile

> models::InstituteProfile update_institute_profile(institute_profile_update)
Update the institute profile (institute_type and/or kapitalmarktorientiert).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**institute_profile_update** | [**InstituteProfileUpdate**](InstituteProfileUpdate.md) |  | [required] |

### Return type

[**models::InstituteProfile**](InstituteProfile.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

