# \ReplenishmentApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**apply_replenishments**](ReplenishmentApi.md#apply_replenishments) | **POST** /api/v1/replenishments/apply | Create one draft stock transfer per (source → target) pair carrying all suggested product lines for that pair.
[**get_replenishments**](ReplenishmentApi.md#get_replenishments) | **GET** /api/v1/replenishments | 



## apply_replenishments

> serde_json::Value apply_replenishments(target_warehouse_id, source_warehouse_id)
Create one draft stock transfer per (source → target) pair carrying all suggested product lines for that pair.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target_warehouse_id** | Option<**String**> | Warehouse to be replenished. Defaults to the tenant's default warehouse. |  |
**source_warehouse_id** | Option<**String**> | Restrict source warehouses to this id. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_replenishments

> models::ReplenishmentResponse get_replenishments(target_warehouse_id, source_warehouse_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target_warehouse_id** | Option<**String**> | Warehouse to be replenished. Defaults to the tenant's default warehouse. |  |
**source_warehouse_id** | Option<**String**> | Restrict source warehouses to this id. |  |

### Return type

[**models::ReplenishmentResponse**](ReplenishmentResponse.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

