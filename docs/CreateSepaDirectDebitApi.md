# \CreateSepaDirectDebitApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_sepa_direct_debit_api**](CreateSepaDirectDebitApi.md#create_sepa_direct_debit_api) | **POST** /api/v1/bookkeeping/sepa-direct-debit | 



## create_sepa_direct_debit_api

> models::SepaDirectDebitResponse create_sepa_direct_debit_api(creditor_name, creditor_iban, creditor_id, mandate_id, mandate_date, debtor_name, debtor_iban, amount, collection_date, creditor_bic, debtor_bic, description)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**creditor_name** | **String** |  | [required] |
**creditor_iban** | **String** |  | [required] |
**creditor_id** | **String** |  | [required] |
**mandate_id** | **String** |  | [required] |
**mandate_date** | **String** |  | [required] |
**debtor_name** | **String** |  | [required] |
**debtor_iban** | **String** |  | [required] |
**amount** | **String** |  | [required] |
**collection_date** | **String** |  | [required] |
**creditor_bic** | Option<**String**> |  |  |
**debtor_bic** | Option<**String**> |  |  |
**description** | Option<**String**> |  |  |

### Return type

[**models::SepaDirectDebitResponse**](SepaDirectDebitResponse.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

