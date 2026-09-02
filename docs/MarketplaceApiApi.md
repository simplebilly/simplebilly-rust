# \MarketplaceApiApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_connection_api**](MarketplaceApiApi.md#create_connection_api) | **POST** /api/v1/marketplace/connections | Create a new connection (for API-key based platforms)
[**delete_connection_api**](MarketplaceApiApi.md#delete_connection_api) | **DELETE** /api/v1/marketplace/connections/{connection_id} | Soft-delete a connection
[**get_connection_api**](MarketplaceApiApi.md#get_connection_api) | **GET** /api/v1/marketplace/connections/{connection_id} | Get a single connection
[**get_sync_direction_api**](MarketplaceApiApi.md#get_sync_direction_api) | **GET** /api/v1/marketplace/connections/{connection_id}/directions | Get current sync direction configuration for a connection
[**get_sync_logs_api**](MarketplaceApiApi.md#get_sync_logs_api) | **GET** /api/v1/marketplace/connections/{connection_id}/logs | Get sync logs for a connection
[**list_connections_api**](MarketplaceApiApi.md#list_connections_api) | **GET** /api/v1/marketplace/connections | List connections for the current tenant
[**list_platforms_api**](MarketplaceApiApi.md#list_platforms_api) | **GET** /api/v1/marketplace/platforms | List all supported platforms
[**oauth_authorize_api**](MarketplaceApiApi.md#oauth_authorize_api) | **POST** /api/v1/marketplace/oauth/authorize | OAuth: initiate authorization flow
[**oauth_callback_api**](MarketplaceApiApi.md#oauth_callback_api) | **POST** /api/v1/marketplace/oauth/callback | OAuth: handle callback after authorization
[**trigger_sync_api**](MarketplaceApiApi.md#trigger_sync_api) | **POST** /api/v1/marketplace/connections/{connection_id}/sync | Trigger sync for a connection
[**update_connection_api**](MarketplaceApiApi.md#update_connection_api) | **PUT** /api/v1/marketplace/connections/{connection_id} | Update a connection
[**update_sync_direction_api**](MarketplaceApiApi.md#update_sync_direction_api) | **PUT** /api/v1/marketplace/connections/{connection_id}/directions | Update per-entity sync direction configuration for a connection
[**webhook_receiver_api**](MarketplaceApiApi.md#webhook_receiver_api) | **POST** /api/v1/marketplace/webhook/{platform}/{connection_id} | Webhook receiver



## create_connection_api

> models::MarketplaceConnection create_connection_api(create_connection_request)
Create a new connection (for API-key based platforms)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_connection_request** | [**CreateConnectionRequest**](CreateConnectionRequest.md) |  | [required] |

### Return type

[**models::MarketplaceConnection**](MarketplaceConnection.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_connection_api

> delete_connection_api(connection_id)
Soft-delete a connection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connection_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_connection_api

> models::MarketplaceConnection get_connection_api(connection_id)
Get a single connection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connection_id** | **String** |  | [required] |

### Return type

[**models::MarketplaceConnection**](MarketplaceConnection.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sync_direction_api

> get_sync_direction_api(connection_id)
Get current sync direction configuration for a connection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connection_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sync_logs_api

> Vec<models::SyncLog> get_sync_logs_api(connection_id)
Get sync logs for a connection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connection_id** | **String** |  | [required] |

### Return type

[**Vec<models::SyncLog>**](SyncLog.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_connections_api

> Vec<models::MarketplaceConnection> list_connections_api()
List connections for the current tenant

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::MarketplaceConnection>**](MarketplaceConnection.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_platforms_api

> Vec<models::PlatformInfo> list_platforms_api()
List all supported platforms

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::PlatformInfo>**](PlatformInfo.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## oauth_authorize_api

> models::OAuthAuthorizeResponse oauth_authorize_api(o_auth_authorize_request)
OAuth: initiate authorization flow

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**o_auth_authorize_request** | [**OAuthAuthorizeRequest**](OAuthAuthorizeRequest.md) |  | [required] |

### Return type

[**models::OAuthAuthorizeResponse**](OAuthAuthorizeResponse.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## oauth_callback_api

> models::MarketplaceConnection oauth_callback_api(o_auth_callback_request)
OAuth: handle callback after authorization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**o_auth_callback_request** | [**OAuthCallbackRequest**](OAuthCallbackRequest.md) |  | [required] |

### Return type

[**models::MarketplaceConnection**](MarketplaceConnection.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trigger_sync_api

> models::SyncSummary trigger_sync_api(connection_id, sync_type, direction)
Trigger sync for a connection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connection_id** | **String** |  | [required] |
**sync_type** | Option<**String**> |  |  |
**direction** | Option<**String**> |  |  |

### Return type

[**models::SyncSummary**](SyncSummary.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_connection_api

> models::MarketplaceConnection update_connection_api(connection_id, update_connection_request)
Update a connection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connection_id** | **String** |  | [required] |
**update_connection_request** | [**UpdateConnectionRequest**](UpdateConnectionRequest.md) |  | [required] |

### Return type

[**models::MarketplaceConnection**](MarketplaceConnection.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_sync_direction_api

> update_sync_direction_api(connection_id, update_sync_direction_request)
Update per-entity sync direction configuration for a connection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connection_id** | **String** |  | [required] |
**update_sync_direction_request** | [**UpdateSyncDirectionRequest**](UpdateSyncDirectionRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## webhook_receiver_api

> webhook_receiver_api(platform, connection_id)
Webhook receiver

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | **String** |  | [required] |
**connection_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

