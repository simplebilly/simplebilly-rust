# \BillingApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_plans**](BillingApi.md#get_plans) | **GET** /api/v1/plans | All canonical plans (free/starter/business/enterprise) — the single source of truth lives in `crate::saasy::plans`, matching marketing.
[**get_quota_api**](BillingApi.md#get_quota_api) | **GET** /api/v1/quota | Effective limits + current usage for the calling tenant.
[**get_subscription_api**](BillingApi.md#get_subscription_api) | **GET** /api/v1/subscription | 
[**get_usage_api**](BillingApi.md#get_usage_api) | **GET** /api/v1/usage | 
[**paddle_subscription_webhook**](BillingApi.md#paddle_subscription_webhook) | **POST** /api/webhooks/paddle/subscription | Paddle Billing subscription webhook. Verifies the `Paddle-Signature` header (HMAC-SHA256 over `\"{ts}:{raw_body}\"` with the webhook secret), then updates `billing_info` and `tenants.plan` for the tenant identified by the subscription `custom_data` (JSON `{\"tenant_id\": \"...\"}` or a bare tenant UUID).
[**put_quota_api**](BillingApi.md#put_quota_api) | **PUT** /api/v1/quota | Write the per-tenant quota override (`admin:settings`). An empty object clears the override.



## get_plans

> models::ApiResponseVecPlan get_plans()
All canonical plans (free/starter/business/enterprise) — the single source of truth lives in `crate::saasy::plans`, matching marketing.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ApiResponseVecPlan**](ApiResponse_Vec_Plan.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_quota_api

> get_quota_api()
Effective limits + current usage for the calling tenant.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_subscription_api

> models::ApiResponseSubscriptionOverview get_subscription_api()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ApiResponseSubscriptionOverview**](ApiResponse_SubscriptionOverview.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_usage_api

> get_usage_api(meter)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**meter** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## paddle_subscription_webhook

> paddle_subscription_webhook()
Paddle Billing subscription webhook. Verifies the `Paddle-Signature` header (HMAC-SHA256 over `\"{ts}:{raw_body}\"` with the webhook secret), then updates `billing_info` and `tenants.plan` for the tenant identified by the subscription `custom_data` (JSON `{\"tenant_id\": \"...\"}` or a bare tenant UUID).

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_quota_api

> put_quota_api(quota_override)
Write the per-tenant quota override (`admin:settings`). An empty object clears the override.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**quota_override** | [**QuotaOverride**](QuotaOverride.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

