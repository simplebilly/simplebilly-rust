# \TenantSettingsApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_tenant_settings**](TenantSettingsApi.md#get_tenant_settings) | **GET** /api/v1/settings/tenant | 
[**update_tenant_settings**](TenantSettingsApi.md#update_tenant_settings) | **PUT** /api/v1/settings/tenant | 



## get_tenant_settings

> models::TenantSettings get_tenant_settings()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::TenantSettings**](TenantSettings.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_tenant_settings

> models::TenantSettings update_tenant_settings(update_tenant_settings)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_tenant_settings** | [**UpdateTenantSettings**](UpdateTenantSettings.md) |  | [required] |

### Return type

[**models::TenantSettings**](TenantSettings.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

