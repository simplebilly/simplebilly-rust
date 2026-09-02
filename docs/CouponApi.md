# \CouponApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**coupon_restore**](CouponApi.md#coupon_restore) | **POST** /api/v1/coupons/{coupon_id}/restore | 
[**create_coupon**](CouponApi.md#create_coupon) | **POST** /api/v1/coupons | 
[**delete_coupon**](CouponApi.md#delete_coupon) | **DELETE** /api/v1/coupons/{coupon_id} | 
[**get_coupon**](CouponApi.md#get_coupon) | **GET** /api/v1/coupons/{coupon_id} | 
[**list_coupons**](CouponApi.md#list_coupons) | **GET** /api/v1/coupons/ | 
[**update_coupon**](CouponApi.md#update_coupon) | **PUT** /api/v1/coupons/{coupon_id} | 



## coupon_restore

> models::Coupon coupon_restore(coupon_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**coupon_id** | **String** |  | [required] |

### Return type

[**models::Coupon**](Coupon.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_coupon

> models::Coupon create_coupon(coupon_create)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**coupon_create** | [**CouponCreate**](CouponCreate.md) |  | [required] |

### Return type

[**models::Coupon**](Coupon.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_coupon

> delete_coupon(coupon_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**coupon_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_coupon

> models::Coupon get_coupon(coupon_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**coupon_id** | **String** |  | [required] |

### Return type

[**models::Coupon**](Coupon.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_coupons

> Vec<models::Coupon> list_coupons(page, page_size, is_active, code, discount_type)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**is_active** | Option<**bool**> |  |  |
**code** | Option<**String**> |  |  |
**discount_type** | Option<**String**> |  |  |

### Return type

[**Vec<models::Coupon>**](Coupon.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_coupon

> models::Coupon update_coupon(coupon_id, coupon_update)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**coupon_id** | **String** |  | [required] |
**coupon_update** | [**CouponUpdate**](CouponUpdate.md) |  | [required] |

### Return type

[**models::Coupon**](Coupon.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

