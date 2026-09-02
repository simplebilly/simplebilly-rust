# \ProductVariantApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_product_variant**](ProductVariantApi.md#create_product_variant) | **POST** /api/v1/product-variants | 
[**delete_product_variant**](ProductVariantApi.md#delete_product_variant) | **DELETE** /api/v1/product-variants/{variant_id} | 
[**generate_product_variants**](ProductVariantApi.md#generate_product_variants) | **POST** /api/v1/product-variants/generate | 
[**get_product_variant**](ProductVariantApi.md#get_product_variant) | **GET** /api/v1/product-variants/{variant_id} | 
[**list_product_variants**](ProductVariantApi.md#list_product_variants) | **GET** /api/v1/product-variants/ | 
[**update_product_variant**](ProductVariantApi.md#update_product_variant) | **PUT** /api/v1/product-variants/{variant_id} | 



## create_product_variant

> models::ProductVariant create_product_variant(product_variant)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_variant** | [**ProductVariant**](ProductVariant.md) |  | [required] |

### Return type

[**models::ProductVariant**](ProductVariant.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_product_variant

> delete_product_variant(variant_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**variant_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## generate_product_variants

> Vec<models::ProductVariant> generate_product_variants(generate_variants_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**generate_variants_request** | [**GenerateVariantsRequest**](GenerateVariantsRequest.md) |  | [required] |

### Return type

[**Vec<models::ProductVariant>**](ProductVariant.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_product_variant

> models::ProductVariant get_product_variant(variant_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**variant_id** | **String** |  | [required] |

### Return type

[**models::ProductVariant**](ProductVariant.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_product_variants

> Vec<models::ProductVariant> list_product_variants(page, page_size, product_id, is_active)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**product_id** | Option<**uuid::Uuid**> |  |  |
**is_active** | Option<**bool**> |  |  |

### Return type

[**Vec<models::ProductVariant>**](ProductVariant.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_product_variant

> models::ProductVariant update_product_variant(variant_id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**variant_id** | **String** |  | [required] |
**body** | Option<**serde_json::Value**> |  | [required] |

### Return type

[**models::ProductVariant**](ProductVariant.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

