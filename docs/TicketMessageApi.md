# \TicketMessageApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_messages_api**](TicketMessageApi.md#list_messages_api) | **GET** /api/v1/support/tickets/{ticket_id}/messages | 
[**send_message_api**](TicketMessageApi.md#send_message_api) | **POST** /api/v1/support/tickets/{ticket_id}/messages | 



## list_messages_api

> Vec<models::TicketMessage> list_messages_api(ticket_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket_id** | **uuid::Uuid** |  | [required] |

### Return type

[**Vec<models::TicketMessage>**](TicketMessage.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_message_api

> models::TicketMessage send_message_api(ticket_id, send_message_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket_id** | **uuid::Uuid** |  | [required] |
**send_message_dto** | [**SendMessageDto**](SendMessageDto.md) |  | [required] |

### Return type

[**models::TicketMessage**](TicketMessage.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

