# \SupportTicketApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_ticket_api**](SupportTicketApi.md#create_ticket_api) | **POST** /api/v1/support/tickets | 
[**delete_ticket_api**](SupportTicketApi.md#delete_ticket_api) | **DELETE** /api/v1/support/tickets/{ticket_id} | 
[**get_ticket_api**](SupportTicketApi.md#get_ticket_api) | **GET** /api/v1/support/tickets/{ticket_id} | 
[**list_tickets_api**](SupportTicketApi.md#list_tickets_api) | **GET** /api/v1/support/tickets | 
[**update_ticket_api**](SupportTicketApi.md#update_ticket_api) | **PUT** /api/v1/support/tickets/{ticket_id} | 



## create_ticket_api

> models::SupportTicket create_ticket_api(create_ticket_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_ticket_request** | [**CreateTicketRequest**](CreateTicketRequest.md) |  | [required] |

### Return type

[**models::SupportTicket**](SupportTicket.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_ticket_api

> delete_ticket_api(ticket_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket_id** | **uuid::Uuid** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ticket_api

> models::SupportTicket get_ticket_api(ticket_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::SupportTicket**](SupportTicket.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_tickets_api

> Vec<models::SupportTicket> list_tickets_api(status, priority, assigned_to, channel_type, customer_id, search, page, page_size)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**status** | Option<**String**> |  |  |
**priority** | Option<**String**> |  |  |
**assigned_to** | Option<**uuid::Uuid**> |  |  |
**channel_type** | Option<**String**> |  |  |
**customer_id** | Option<**String**> |  |  |
**search** | Option<**String**> |  |  |
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |

### Return type

[**Vec<models::SupportTicket>**](SupportTicket.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_ticket_api

> models::SupportTicket update_ticket_api(ticket_id, support_ticket_update)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket_id** | **uuid::Uuid** |  | [required] |
**support_ticket_update** | [**SupportTicketUpdate**](SupportTicketUpdate.md) |  | [required] |

### Return type

[**models::SupportTicket**](SupportTicket.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

