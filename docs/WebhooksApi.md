# \WebhooksApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_subscription**](WebhooksApi.md#create_subscription) | **POST** /api/v1/webhook-subscriptions | Create a webhook subscription (outbound hook).
[**delete_subscription**](WebhooksApi.md#delete_subscription) | **DELETE** /api/v1/webhook-subscriptions/{subscription_id} | Delete a webhook subscription.
[**emit_api**](WebhooksApi.md#emit_api) | **POST** /api/v1/webhooks/emit | Manually fire an event against matching hooks (for testing/flows).
[**list_event**](WebhooksApi.md#list_event) | **GET** /api/v1/webhook-events | List webhook events (inbound + outbound log).
[**list_subscriptions**](WebhooksApi.md#list_subscriptions) | **GET** /api/v1/webhook-subscriptions | List webhook subscriptions for the tenant.
[**update_subscription**](WebhooksApi.md#update_subscription) | **PUT** /api/v1/webhook-subscriptions/{subscription_id} | Update a webhook subscription.



## create_subscription

> models::WebhookSubscription create_subscription(create_subscription_request)
Create a webhook subscription (outbound hook).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_subscription_request** | [**CreateSubscriptionRequest**](CreateSubscriptionRequest.md) |  | [required] |

### Return type

[**models::WebhookSubscription**](WebhookSubscription.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_subscription

> delete_subscription(subscription_id)
Delete a webhook subscription.

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
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## emit_api

> emit_api(emit_event_request)
Manually fire an event against matching hooks (for testing/flows).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**emit_event_request** | [**EmitEventRequest**](EmitEventRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_event

> Vec<models::WebhookEvent> list_event()
List webhook events (inbound + outbound log).

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::WebhookEvent>**](WebhookEvent.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_subscriptions

> Vec<models::WebhookSubscription> list_subscriptions()
List webhook subscriptions for the tenant.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::WebhookSubscription>**](WebhookSubscription.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_subscription

> models::WebhookSubscription update_subscription(subscription_id, update_subscription_request)
Update a webhook subscription.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | **String** |  | [required] |
**update_subscription_request** | [**UpdateSubscriptionRequest**](UpdateSubscriptionRequest.md) |  | [required] |

### Return type

[**models::WebhookSubscription**](WebhookSubscription.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

