# \EventSubscriptionApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_event_subscription**](EventSubscriptionApi.md#create_event_subscription) | **POST** /api/v1/event-subscriptions | 
[**delete_event_subscription**](EventSubscriptionApi.md#delete_event_subscription) | **DELETE** /api/v1/event-subscriptions/{subscription_id} | 
[**list_event_subscriptions**](EventSubscriptionApi.md#list_event_subscriptions) | **GET** /api/v1/event-subscriptions/ | 



## create_event_subscription

> models::EventSubscription create_event_subscription(body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<**serde_json::Value**> |  | [required] |

### Return type

[**models::EventSubscription**](EventSubscription.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_event_subscription

> delete_event_subscription(subscription_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_event_subscriptions

> Vec<models::EventSubscription> list_event_subscriptions()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::EventSubscription>**](EventSubscription.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

