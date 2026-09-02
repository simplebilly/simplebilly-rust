# \VoucherApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_voucher**](VoucherApi.md#create_voucher) | **POST** /api/v1/vouchers | 
[**delete_voucher**](VoucherApi.md#delete_voucher) | **DELETE** /api/v1/vouchers/{voucher_id} | 
[**get_voucher**](VoucherApi.md#get_voucher) | **GET** /api/v1/vouchers/{voucher_id} | 
[**list_vouchers**](VoucherApi.md#list_vouchers) | **GET** /api/v1/vouchers/ | 
[**update_voucher**](VoucherApi.md#update_voucher) | **PUT** /api/v1/vouchers/{voucher_id} | 
[**voucher_restore**](VoucherApi.md#voucher_restore) | **POST** /api/v1/vouchers/{voucher_id}/restore | 



## create_voucher

> models::Voucher create_voucher(voucher_create)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**voucher_create** | [**VoucherCreate**](VoucherCreate.md) |  | [required] |

### Return type

[**models::Voucher**](Voucher.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_voucher

> delete_voucher(voucher_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**voucher_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_voucher

> models::Voucher get_voucher(voucher_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**voucher_id** | **String** |  | [required] |

### Return type

[**models::Voucher**](Voucher.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_vouchers

> Vec<models::Voucher> list_vouchers(page, page_size, voucher_type, voucher_status, contact_name, date_from, date_to)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**voucher_type** | Option<**String**> |  |  |
**voucher_status** | Option<**String**> |  |  |
**contact_name** | Option<**String**> |  |  |
**date_from** | Option<**chrono::NaiveDate**> |  |  |
**date_to** | Option<**chrono::NaiveDate**> |  |  |

### Return type

[**Vec<models::Voucher>**](Voucher.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_voucher

> models::Voucher update_voucher(voucher_id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**voucher_id** | **String** |  | [required] |
**body** | Option<**serde_json::Value**> |  | [required] |

### Return type

[**models::Voucher**](Voucher.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## voucher_restore

> models::Voucher voucher_restore(voucher_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**voucher_id** | **String** |  | [required] |

### Return type

[**models::Voucher**](Voucher.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

