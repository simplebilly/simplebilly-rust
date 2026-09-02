# \ShippingThresholdApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_shipping_threshold**](ShippingThresholdApi.md#create_shipping_threshold) | **POST** /api/v1/shipping-thresholds | 
[**delete_shipping_threshold**](ShippingThresholdApi.md#delete_shipping_threshold) | **DELETE** /api/v1/shipping-thresholds/{threshold_id} | 
[**get_deliverable**](ShippingThresholdApi.md#get_deliverable) | **GET** /api/v1/shipping-thresholds/deliverable | 
[**get_shipping_threshold**](ShippingThresholdApi.md#get_shipping_threshold) | **GET** /api/v1/shipping-thresholds/{threshold_id} | 
[**list_shipping_thresholds**](ShippingThresholdApi.md#list_shipping_thresholds) | **GET** /api/v1/shipping-thresholds/ | 
[**update_shipping_threshold**](ShippingThresholdApi.md#update_shipping_threshold) | **PUT** /api/v1/shipping-thresholds/{threshold_id} | 



## create_shipping_threshold

> models::ShippingThreshold create_shipping_threshold(shipping_threshold_create)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**shipping_threshold_create** | [**ShippingThresholdCreate**](ShippingThresholdCreate.md) |  | [required] |

### Return type

[**models::ShippingThreshold**](ShippingThreshold.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_shipping_threshold

> delete_shipping_threshold(threshold_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**threshold_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_deliverable

> models::DeliverableResponse get_deliverable(product_id, warehouse_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **uuid::Uuid** |  | [required] |
**warehouse_id** | Option<**String**> |  |  |

### Return type

[**models::DeliverableResponse**](DeliverableResponse.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_shipping_threshold

> models::ShippingThreshold get_shipping_threshold(threshold_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**threshold_id** | **String** |  | [required] |

### Return type

[**models::ShippingThreshold**](ShippingThreshold.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_shipping_thresholds

> Vec<models::ShippingThreshold> list_shipping_thresholds(page, page_size, product_id, warehouse_id, is_active)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**product_id** | Option<**uuid::Uuid**> |  |  |
**warehouse_id** | Option<**String**> |  |  |
**is_active** | Option<**bool**> |  |  |

### Return type

[**Vec<models::ShippingThreshold>**](ShippingThreshold.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_shipping_threshold

> models::ShippingThreshold update_shipping_threshold(threshold_id, shipping_threshold_update)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**threshold_id** | **String** |  | [required] |
**shipping_threshold_update** | [**ShippingThresholdUpdate**](ShippingThresholdUpdate.md) |  | [required] |

### Return type

[**models::ShippingThreshold**](ShippingThreshold.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

