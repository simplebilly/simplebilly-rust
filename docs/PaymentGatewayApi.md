# \PaymentGatewayApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_payment_gateway_api**](PaymentGatewayApi.md#create_payment_gateway_api) | **POST** /api/v1/payment-gateways | 
[**delete_payment_gateway_api**](PaymentGatewayApi.md#delete_payment_gateway_api) | **DELETE** /api/v1/payment-gateways/{gateway_id} | 
[**list_payment_gateways_api**](PaymentGatewayApi.md#list_payment_gateways_api) | **GET** /api/v1/payment-gateways/ | 
[**oauth_authorize_api**](PaymentGatewayApi.md#oauth_authorize_api) | **POST** /api/v1/payment-gateways/oauth/authorize | 
[**oauth_callback_api**](PaymentGatewayApi.md#oauth_callback_api) | **POST** /api/v1/payment-gateways/oauth/callback | 
[**update_payment_gateway_api**](PaymentGatewayApi.md#update_payment_gateway_api) | **PUT** /api/v1/payment-gateways/{gateway_id} | 



## create_payment_gateway_api

> models::PaymentGateway create_payment_gateway_api(body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<**serde_json::Value**> |  | [required] |

### Return type

[**models::PaymentGateway**](PaymentGateway.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_payment_gateway_api

> delete_payment_gateway_api(gateway_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gateway_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_payment_gateways_api

> Vec<models::PaymentGateway> list_payment_gateways_api()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::PaymentGateway>**](PaymentGateway.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## oauth_authorize_api

> models::GatewayOAuthAuthorizeResponse oauth_authorize_api(gateway_o_auth_authorize_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gateway_o_auth_authorize_request** | [**GatewayOAuthAuthorizeRequest**](GatewayOAuthAuthorizeRequest.md) |  | [required] |

### Return type

[**models::GatewayOAuthAuthorizeResponse**](GatewayOAuthAuthorizeResponse.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## oauth_callback_api

> models::PaymentGateway oauth_callback_api(gateway_o_auth_callback_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gateway_o_auth_callback_request** | [**GatewayOAuthCallbackRequest**](GatewayOAuthCallbackRequest.md) |  | [required] |

### Return type

[**models::PaymentGateway**](PaymentGateway.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_payment_gateway_api

> models::PaymentGateway update_payment_gateway_api(gateway_id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gateway_id** | **String** |  | [required] |
**body** | Option<**serde_json::Value**> |  | [required] |

### Return type

[**models::PaymentGateway**](PaymentGateway.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

