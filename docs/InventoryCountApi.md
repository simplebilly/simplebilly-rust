# \InventoryCountApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_inventory_count**](InventoryCountApi.md#create_inventory_count) | **POST** /api/v1/inventory-counts | 
[**delete_inventory_count**](InventoryCountApi.md#delete_inventory_count) | **DELETE** /api/v1/inventory-counts/{inventory_count_id} | 
[**generate_inventory_count**](InventoryCountApi.md#generate_inventory_count) | **POST** /api/v1/inventory-counts/generate | 
[**get_inventory_count**](InventoryCountApi.md#get_inventory_count) | **GET** /api/v1/inventory-counts/{inventory_count_id} | 
[**list_inventory_counts**](InventoryCountApi.md#list_inventory_counts) | **GET** /api/v1/inventory-counts/ | 
[**update_inventory_count**](InventoryCountApi.md#update_inventory_count) | **PUT** /api/v1/inventory-counts/{inventory_count_id} | 
[**update_inventory_count_status**](InventoryCountApi.md#update_inventory_count_status) | **PUT** /api/v1/inventory-counts/{inventory_count_id}/status | 



## create_inventory_count

> models::InventoryCount create_inventory_count(inventory_count)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inventory_count** | [**InventoryCount**](InventoryCount.md) |  | [required] |

### Return type

[**models::InventoryCount**](InventoryCount.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_inventory_count

> delete_inventory_count(inventory_count_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inventory_count_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## generate_inventory_count

> models::InventoryCount generate_inventory_count(generate_count_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**generate_count_request** | [**GenerateCountRequest**](GenerateCountRequest.md) |  | [required] |

### Return type

[**models::InventoryCount**](InventoryCount.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_inventory_count

> models::InventoryCount get_inventory_count(inventory_count_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inventory_count_id** | **String** |  | [required] |

### Return type

[**models::InventoryCount**](InventoryCount.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_inventory_counts

> Vec<models::InventoryCount> list_inventory_counts(page, page_size, status, warehouse_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**status** | Option<**String**> |  |  |
**warehouse_id** | Option<**String**> |  |  |

### Return type

[**Vec<models::InventoryCount>**](InventoryCount.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_inventory_count

> models::InventoryCount update_inventory_count(inventory_count_id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inventory_count_id** | **String** |  | [required] |
**body** | Option<**serde_json::Value**> |  | [required] |

### Return type

[**models::InventoryCount**](InventoryCount.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_inventory_count_status

> models::InventoryCount update_inventory_count_status(inventory_count_id, inventory_count_status_update)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inventory_count_id** | **String** |  | [required] |
**inventory_count_status_update** | [**InventoryCountStatusUpdate**](InventoryCountStatusUpdate.md) |  | [required] |

### Return type

[**models::InventoryCount**](InventoryCount.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

