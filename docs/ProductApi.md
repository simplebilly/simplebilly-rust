# \ProductApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_product_api**](ProductApi.md#create_product_api) | **POST** /api/v1/products | 
[**delete_product_api**](ProductApi.md#delete_product_api) | **DELETE** /api/v1/products/{product_id} | 
[**get_product_api**](ProductApi.md#get_product_api) | **GET** /api/v1/products/{product_id} | 
[**get_product_stock_api**](ProductApi.md#get_product_stock_api) | **GET** /api/v1/products/{product_id}/stock | 
[**get_products_api**](ProductApi.md#get_products_api) | **GET** /api/v1/products/ | 
[**list_low_stock_products_api**](ProductApi.md#list_low_stock_products_api) | **GET** /api/v1/products/low-stock | 
[**product_restore**](ProductApi.md#product_restore) | **POST** /api/v1/products/{product_id}/restore | 
[**update_product_api**](ProductApi.md#update_product_api) | **PUT** /api/v1/products/{product_id} | 
[**update_product_stock_api**](ProductApi.md#update_product_stock_api) | **PUT** /api/v1/products/{product_id}/stock | 



## create_product_api

> models::Product create_product_api(product_create)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_create** | [**ProductCreate**](ProductCreate.md) |  | [required] |

### Return type

[**models::Product**](Product.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_product_api

> delete_product_api(product_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **uuid::Uuid** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_product_api

> models::Product get_product_api(product_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::Product**](Product.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_product_stock_api

> models::ProductStock get_product_stock_api(product_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::ProductStock**](ProductStock.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_products_api

> Vec<models::Product> get_products_api(page, page_size, search, include_deleted)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**search** | Option<**String**> |  |  |
**include_deleted** | Option<**bool**> | Soft-delete entities: set true to include rows with `deleted_at` set. |  |

### Return type

[**Vec<models::Product>**](Product.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_low_stock_products_api

> Vec<models::ProductStock> list_low_stock_products_api(threshold)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**threshold** | Option<**i64**> |  |  |

### Return type

[**Vec<models::ProductStock>**](ProductStock.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## product_restore

> models::Product product_restore(product_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::Product**](Product.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_product_api

> models::Product update_product_api(product_id, product_update)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **uuid::Uuid** |  | [required] |
**product_update** | [**ProductUpdate**](ProductUpdate.md) |  | [required] |

### Return type

[**models::Product**](Product.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_product_stock_api

> models::ProductStock update_product_stock_api(product_id, stock_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **uuid::Uuid** |  | [required] |
**stock_update_request** | [**StockUpdateRequest**](StockUpdateRequest.md) |  | [required] |

### Return type

[**models::ProductStock**](ProductStock.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

