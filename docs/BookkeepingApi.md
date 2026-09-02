# \BookkeepingApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**allocate_payment_api**](BookkeepingApi.md#allocate_payment_api) | **POST** /api/v1/payments/allocate | Allocate a payment to an invoice
[**bwa_report_api**](BookkeepingApi.md#bwa_report_api) | **GET** /api/v1/bookkeeping/bwa | Get BWA (Betriebswirtschaftliche Auswertung) report
[**elster_status_api**](BookkeepingApi.md#elster_status_api) | **GET** /api/v1/bookkeeping/elster/status | 
[**elster_validate_api**](BookkeepingApi.md#elster_validate_api) | **POST** /api/v1/bookkeeping/ustva/elster-validate | 
[**elster_xml_api**](BookkeepingApi.md#elster_xml_api) | **GET** /api/v1/bookkeeping/ustva/elster-xml | 
[**get_cashflow**](BookkeepingApi.md#get_cashflow) | **GET** /api/v1/bookkeeping/cashflow | GET /api/v1/bookkeeping/cashflow Returns operating, investing, and financing cashflow for the given period.
[**get_liquidity**](BookkeepingApi.md#get_liquidity) | **GET** /api/v1/bookkeeping/liquidity | GET /api/v1/bookkeeping/liquidity Returns current liquidity position with ratios.
[**get_open_invoices_api**](BookkeepingApi.md#get_open_invoices_api) | **GET** /api/v1/payments/open-invoices/{customer_id} | Get open invoices for a customer
[**get_verfahrensdokumentation**](BookkeepingApi.md#get_verfahrensdokumentation) | **GET** /api/v1/bookkeeping/verfahrensdokumentation | GET /api/v1/bookkeeping/verfahrensdokumentation Returns the complete compliance catalog of all documented modules.
[**run_dunning_api**](BookkeepingApi.md#run_dunning_api) | **POST** /api/v1/bookkeeping/dunning | 



## allocate_payment_api

> allocate_payment_api(allocate_payment_request)
Allocate a payment to an invoice

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**allocate_payment_request** | [**AllocatePaymentRequest**](AllocatePaymentRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bwa_report_api

> models::BwaReport bwa_report_api(year, month)
Get BWA (Betriebswirtschaftliche Auswertung) report

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | Option<**i32**> |  |  |
**month** | Option<**i32**> |  |  |

### Return type

[**models::BwaReport**](BWAReport.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## elster_status_api

> models::ElsterStatus elster_status_api()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ElsterStatus**](ElsterStatus.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## elster_validate_api

> elster_validate_api(zeitraum)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zeitraum** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## elster_xml_api

> elster_xml_api(zeitraum)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zeitraum** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cashflow

> models::CashflowReport get_cashflow(year, month)
GET /api/v1/bookkeeping/cashflow Returns operating, investing, and financing cashflow for the given period.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | Option<**i32**> |  |  |
**month** | Option<**i32**> |  |  |

### Return type

[**models::CashflowReport**](CashflowReport.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_liquidity

> models::LiquidityPosition get_liquidity()
GET /api/v1/bookkeeping/liquidity Returns current liquidity position with ratios.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::LiquidityPosition**](LiquidityPosition.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_open_invoices_api

> Vec<models::Invoice> get_open_invoices_api(customer_id)
Get open invoices for a customer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_id** | **String** |  | [required] |

### Return type

[**Vec<models::Invoice>**](Invoice.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_verfahrensdokumentation

> models::Verfahrensdokumentation get_verfahrensdokumentation()
GET /api/v1/bookkeeping/verfahrensdokumentation Returns the complete compliance catalog of all documented modules.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Verfahrensdokumentation**](Verfahrensdokumentation.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## run_dunning_api

> models::DunningResult run_dunning_api()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::DunningResult**](DunningResult.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

