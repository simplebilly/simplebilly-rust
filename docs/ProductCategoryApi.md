# \ProductCategoryApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_product_category**](ProductCategoryApi.md#create_product_category) | **POST** /api/v1/product-categories | 
[**delete_product_category**](ProductCategoryApi.md#delete_product_category) | **DELETE** /api/v1/product-categories/{category_id} | 
[**get_product_category**](ProductCategoryApi.md#get_product_category) | **GET** /api/v1/product-categories/{category_id} | 
[**list_product_categories**](ProductCategoryApi.md#list_product_categories) | **GET** /api/v1/product-categories | 
[**update_product_category**](ProductCategoryApi.md#update_product_category) | **PUT** /api/v1/product-categories/{category_id} | 



## create_product_category

> models::ProductCategory create_product_category(product_category)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_category** | [**ProductCategory**](ProductCategory.md) |  | [required] |

### Return type

[**models::ProductCategory**](ProductCategory.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_product_category

> delete_product_category(category_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**category_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_product_category

> models::ProductCategory get_product_category(category_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**category_id** | **String** |  | [required] |

### Return type

[**models::ProductCategory**](ProductCategory.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_product_categories

> Vec<models::ProductCategory> list_product_categories()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ProductCategory>**](ProductCategory.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_product_category

> models::ProductCategory update_product_category(category_id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**category_id** | **String** |  | [required] |
**body** | Option<**serde_json::Value**> |  | [required] |

### Return type

[**models::ProductCategory**](ProductCategory.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

