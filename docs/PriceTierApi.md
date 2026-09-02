# \PriceTierApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_price_tier**](PriceTierApi.md#create_price_tier) | **POST** /api/v1/price-tiers | 
[**delete_price_tier**](PriceTierApi.md#delete_price_tier) | **DELETE** /api/v1/price-tiers/{price_tier_id} | 
[**get_price_tier**](PriceTierApi.md#get_price_tier) | **GET** /api/v1/price-tiers/{price_tier_id} | 
[**get_resolved_price**](PriceTierApi.md#get_resolved_price) | **GET** /api/v1/price-tiers/resolved | 
[**list_price_tiers**](PriceTierApi.md#list_price_tiers) | **GET** /api/v1/price-tiers/ | 
[**update_price_tier**](PriceTierApi.md#update_price_tier) | **PUT** /api/v1/price-tiers/{price_tier_id} | 



## create_price_tier

> models::PriceTier create_price_tier(price_tier_create)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**price_tier_create** | [**PriceTierCreate**](PriceTierCreate.md) |  | [required] |

### Return type

[**models::PriceTier**](PriceTier.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_price_tier

> delete_price_tier(price_tier_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**price_tier_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_price_tier

> models::PriceTier get_price_tier(price_tier_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**price_tier_id** | **String** |  | [required] |

### Return type

[**models::PriceTier**](PriceTier.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_resolved_price

> models::ResolvedPriceResponse get_resolved_price(product_id, quantity, contact_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **uuid::Uuid** |  | [required] |
**quantity** | Option<**i64**> |  |  |
**contact_id** | Option<**String**> | Contact used to match customer-group-scoped tiers. |  |

### Return type

[**models::ResolvedPriceResponse**](ResolvedPriceResponse.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_price_tiers

> Vec<models::PriceTier> list_price_tiers(page, page_size, product_id, customer_group_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**product_id** | Option<**uuid::Uuid**> |  |  |
**customer_group_id** | Option<**String**> |  |  |

### Return type

[**Vec<models::PriceTier>**](PriceTier.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_price_tier

> models::PriceTier update_price_tier(price_tier_id, price_tier_update)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**price_tier_id** | **String** |  | [required] |
**price_tier_update** | [**PriceTierUpdate**](PriceTierUpdate.md) |  | [required] |

### Return type

[**models::PriceTier**](PriceTier.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

