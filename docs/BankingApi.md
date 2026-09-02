# \BankingApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bank_lookup_api**](BankingApi.md#bank_lookup_api) | **GET** /api/v1/bookkeeping/banking/lookup | 
[**bank_transactions_api**](BankingApi.md#bank_transactions_api) | **GET** /api/v1/bookkeeping/banking/transactions | 
[**hebesatz_lookup_api**](BankingApi.md#hebesatz_lookup_api) | **GET** /api/v1/bookkeeping/hebesatz | 



## bank_lookup_api

> models::BankLookup bank_lookup_api(iban)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**iban** | **String** |  | [required] |

### Return type

[**models::BankLookup**](BankLookup.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bank_transactions_api

> bank_transactions_api()


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


## hebesatz_lookup_api

> Vec<models::HebesatzLookup> hebesatz_lookup_api(gemeindeschluessel, plz, name, stichtag, country_code)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gemeindeschluessel** | Option<**String**> |  |  |
**plz** | Option<**String**> |  |  |
**name** | Option<**String**> |  |  |
**stichtag** | Option<**String**> | Stichtag for validity (YYYY-MM-DD); defaults to today. Picks row where valid_from <= date <= valid_to. |  |
**country_code** | Option<**String**> |  |  |

### Return type

[**Vec<models::HebesatzLookup>**](HebesatzLookup.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

