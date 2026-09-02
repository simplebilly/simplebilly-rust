# \ProductAttributeApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_product_attribute**](ProductAttributeApi.md#create_product_attribute) | **POST** /api/v1/product-attributes | 
[**delete_product_attribute**](ProductAttributeApi.md#delete_product_attribute) | **DELETE** /api/v1/product-attributes/{attribute_id} | 
[**get_product_attribute**](ProductAttributeApi.md#get_product_attribute) | **GET** /api/v1/product-attributes/{attribute_id} | 
[**list_product_attributes**](ProductAttributeApi.md#list_product_attributes) | **GET** /api/v1/product-attributes/ | 
[**update_product_attribute**](ProductAttributeApi.md#update_product_attribute) | **PUT** /api/v1/product-attributes/{attribute_id} | 



## create_product_attribute

> models::ProductAttribute create_product_attribute(product_attribute_create)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_attribute_create** | [**ProductAttributeCreate**](ProductAttributeCreate.md) |  | [required] |

### Return type

[**models::ProductAttribute**](ProductAttribute.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_product_attribute

> delete_product_attribute(attribute_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**attribute_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_product_attribute

> models::ProductAttribute get_product_attribute(attribute_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**attribute_id** | **String** |  | [required] |

### Return type

[**models::ProductAttribute**](ProductAttribute.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_product_attributes

> Vec<models::ProductAttribute> list_product_attributes(page, page_size, product_id, is_filterable, search)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**product_id** | Option<**uuid::Uuid**> |  |  |
**is_filterable** | Option<**bool**> |  |  |
**search** | Option<**String**> |  |  |

### Return type

[**Vec<models::ProductAttribute>**](ProductAttribute.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_product_attribute

> models::ProductAttribute update_product_attribute(attribute_id, product_attribute_update)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**attribute_id** | **String** |  | [required] |
**product_attribute_update** | [**ProductAttributeUpdate**](ProductAttributeUpdate.md) |  | [required] |

### Return type

[**models::ProductAttribute**](ProductAttribute.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

