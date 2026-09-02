# \ShippingApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_credentials_api**](ShippingApi.md#get_credentials_api) | **GET** /api/v1/shipping/credentials | 
[**get_rates_api**](ShippingApi.md#get_rates_api) | **POST** /api/v1/shipping/rates | 
[**list_providers_api**](ShippingApi.md#list_providers_api) | **GET** /api/v1/shipping/providers | 
[**save_credentials_api**](ShippingApi.md#save_credentials_api) | **PUT** /api/v1/shipping/credentials | 



## get_credentials_api

> models::ShippingCredentials get_credentials_api()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ShippingCredentials**](ShippingCredentials.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_rates_api

> models::RateResponse get_rates_api(rate_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rate_request** | [**RateRequest**](RateRequest.md) |  | [required] |

### Return type

[**models::RateResponse**](RateResponse.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_providers_api

> Vec<models::ProviderInfo> list_providers_api()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ProviderInfo>**](ProviderInfo.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## save_credentials_api

> models::ShippingCredentials save_credentials_api(shipping_credentials)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**shipping_credentials** | [**ShippingCredentials**](ShippingCredentials.md) |  | [required] |

### Return type

[**models::ShippingCredentials**](ShippingCredentials.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

