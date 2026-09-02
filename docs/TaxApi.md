# \TaxApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_tax_rate**](TaxApi.md#create_tax_rate) | **POST** /api/v1/tax-rates | Create a tax rate (`admin:settings`).
[**delete_tax_rate**](TaxApi.md#delete_tax_rate) | **DELETE** /api/v1/tax-rates/{id} | Delete a tax rate by id (`admin:settings`).
[**list_tax_rates**](TaxApi.md#list_tax_rates) | **GET** /api/v1/tax-rates | List the calling tenant's tax rates.
[**update_tax_rate**](TaxApi.md#update_tax_rate) | **PUT** /api/v1/tax-rates/{id} | Update a tax rate by id (`admin:settings`). Replaces all body fields.



## create_tax_rate

> create_tax_rate(tax_rate_create)
Create a tax rate (`admin:settings`).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tax_rate_create** | [**TaxRateCreate**](TaxRateCreate.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_tax_rate

> delete_tax_rate(id)
Delete a tax rate by id (`admin:settings`).

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
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_tax_rates

> list_tax_rates()
List the calling tenant's tax rates.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_tax_rate

> update_tax_rate(id, tax_rate_create)
Update a tax rate by id (`admin:settings`). Replaces all body fields.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**tax_rate_create** | [**TaxRateCreate**](TaxRateCreate.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

