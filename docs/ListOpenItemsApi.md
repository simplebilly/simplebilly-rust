# \ListOpenItemsApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_open_items_api**](ListOpenItemsApi.md#list_open_items_api) | **GET** /api/v1/bookkeeping/open-items | 



## list_open_items_api

> Vec<models::OpenItem> list_open_items_api(reminder_level1_days, reminder_level2_days, reminder_level3_days, customer_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reminder_level1_days** | Option<**i64**> |  |  |
**reminder_level2_days** | Option<**i64**> |  |  |
**reminder_level3_days** | Option<**i64**> |  |  |
**customer_id** | Option<**String**> |  |  |

### Return type

[**Vec<models::OpenItem>**](OpenItem.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

