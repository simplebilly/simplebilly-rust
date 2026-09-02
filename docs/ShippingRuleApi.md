# \ShippingRuleApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_shipping_rule**](ShippingRuleApi.md#create_shipping_rule) | **POST** /api/v1/shipping-rules | 
[**delete_shipping_rule**](ShippingRuleApi.md#delete_shipping_rule) | **DELETE** /api/v1/shipping-rules/{rule_id} | 
[**get_shipping_rule**](ShippingRuleApi.md#get_shipping_rule) | **GET** /api/v1/shipping-rules/{rule_id} | 
[**list_shipping_rules**](ShippingRuleApi.md#list_shipping_rules) | **GET** /api/v1/shipping-rules/ | 
[**update_shipping_rule**](ShippingRuleApi.md#update_shipping_rule) | **PUT** /api/v1/shipping-rules/{rule_id} | 



## create_shipping_rule

> models::ShippingRule create_shipping_rule(shipping_rule_create)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**shipping_rule_create** | [**ShippingRuleCreate**](ShippingRuleCreate.md) |  | [required] |

### Return type

[**models::ShippingRule**](ShippingRule.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_shipping_rule

> delete_shipping_rule(rule_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_shipping_rule

> models::ShippingRule get_shipping_rule(rule_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_id** | **String** |  | [required] |

### Return type

[**models::ShippingRule**](ShippingRule.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_shipping_rules

> Vec<models::ShippingRule> list_shipping_rules(page, page_size, country)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**country** | Option<**String**> |  |  |

### Return type

[**Vec<models::ShippingRule>**](ShippingRule.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_shipping_rule

> models::ShippingRule update_shipping_rule(rule_id, shipping_rule_update)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_id** | **String** |  | [required] |
**shipping_rule_update** | [**ShippingRuleUpdate**](ShippingRuleUpdate.md) |  | [required] |

### Return type

[**models::ShippingRule**](ShippingRule.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

