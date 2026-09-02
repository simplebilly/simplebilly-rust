# \OrderApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_order_tags**](OrderApi.md#add_order_tags) | **POST** /api/v1/orders/{order_id}/tags | 
[**find_order_by_external_ref**](OrderApi.md#find_order_by_external_ref) | **GET** /api/v1/orders/by-ext-ref/{ext_ref} | 
[**get_order**](OrderApi.md#get_order) | **GET** /api/v1/order/{order_number} | 
[**get_orders**](OrderApi.md#get_orders) | **GET** /api/v1/orders | 
[**patch_order**](OrderApi.md#patch_order) | **PATCH** /api/v1/orders/{order_id} | 
[**replace_order_tags**](OrderApi.md#replace_order_tags) | **PUT** /api/v1/orders/{order_id}/tags | 
[**update_order_state**](OrderApi.md#update_order_state) | **PUT** /api/v1/orders/{order_id}/state | 



## add_order_tags

> models::Order add_order_tags(order_id, order_tags_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_id** | **String** |  | [required] |
**order_tags_request** | [**OrderTagsRequest**](OrderTagsRequest.md) |  | [required] |

### Return type

[**models::Order**](Order.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_order_by_external_ref

> models::Order find_order_by_external_ref(ext_ref)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ext_ref** | **String** |  | [required] |

### Return type

[**models::Order**](Order.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_order

> models::Order get_order(order_number)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_number** | **String** |  | [required] |

### Return type

[**models::Order**](Order.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_orders

> Vec<models::Order> get_orders(page, page_size, search, include_deleted)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**search** | Option<**String**> |  |  |
**include_deleted** | Option<**bool**> | Soft-delete entities: set true to include rows with `deleted_at` set. |  |

### Return type

[**Vec<models::Order>**](Order.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_order

> models::Order patch_order(order_id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_id** | **String** |  | [required] |
**body** | Option<**serde_json::Value**> |  | [required] |

### Return type

[**models::Order**](Order.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_order_tags

> models::Order replace_order_tags(order_id, order_tags_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_id** | **String** |  | [required] |
**order_tags_request** | [**OrderTagsRequest**](OrderTagsRequest.md) |  | [required] |

### Return type

[**models::Order**](Order.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_order_state

> models::Order update_order_state(order_id, order_state_update)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_id** | **String** |  | [required] |
**order_state_update** | [**OrderStateUpdate**](OrderStateUpdate.md) |  | [required] |

### Return type

[**models::Order**](Order.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

