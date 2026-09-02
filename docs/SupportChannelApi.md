# \SupportChannelApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_channel_api**](SupportChannelApi.md#create_channel_api) | **POST** /api/v1/support/channels | 
[**delete_channel_api**](SupportChannelApi.md#delete_channel_api) | **DELETE** /api/v1/support/channels/{channel_id} | 
[**list_channels_api**](SupportChannelApi.md#list_channels_api) | **GET** /api/v1/support/channels | 
[**update_channel_api**](SupportChannelApi.md#update_channel_api) | **PUT** /api/v1/support/channels/{channel_id} | 



## create_channel_api

> models::SupportChannel create_channel_api(create_channel_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_channel_dto** | [**CreateChannelDto**](CreateChannelDto.md) |  | [required] |

### Return type

[**models::SupportChannel**](SupportChannel.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_channel_api

> delete_channel_api(channel_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **uuid::Uuid** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_channels_api

> Vec<models::SupportChannel> list_channels_api()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::SupportChannel>**](SupportChannel.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_channel_api

> models::SupportChannel update_channel_api(channel_id, update_channel_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **uuid::Uuid** |  | [required] |
**update_channel_dto** | [**UpdateChannelDto**](UpdateChannelDto.md) |  | [required] |

### Return type

[**models::SupportChannel**](SupportChannel.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

