# \CustomerCommunicationApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_communication**](CustomerCommunicationApi.md#create_communication) | **POST** /api/v1/communications | 
[**customercommunication_restore**](CustomerCommunicationApi.md#customercommunication_restore) | **POST** /api/v1/communications/{communication_id}/restore | 
[**delete_communication**](CustomerCommunicationApi.md#delete_communication) | **DELETE** /api/v1/communications/{communication_id} | 
[**get_communication**](CustomerCommunicationApi.md#get_communication) | **GET** /api/v1/communications/{communication_id} | 
[**get_contact_history**](CustomerCommunicationApi.md#get_contact_history) | **GET** /api/v1/contacts/{contact_id}/communications | 
[**list_communications**](CustomerCommunicationApi.md#list_communications) | **GET** /api/v1/communications/ | 
[**update_communication**](CustomerCommunicationApi.md#update_communication) | **PUT** /api/v1/communications/{communication_id} | 



## create_communication

> models::CustomerCommunication create_communication(customer_communication_create)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_communication_create** | [**CustomerCommunicationCreate**](CustomerCommunicationCreate.md) |  | [required] |

### Return type

[**models::CustomerCommunication**](CustomerCommunication.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## customercommunication_restore

> models::CustomerCommunication customercommunication_restore(communication_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**communication_id** | **String** |  | [required] |

### Return type

[**models::CustomerCommunication**](CustomerCommunication.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_communication

> delete_communication(communication_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**communication_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_communication

> models::CustomerCommunication get_communication(communication_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**communication_id** | **String** |  | [required] |

### Return type

[**models::CustomerCommunication**](CustomerCommunication.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_contact_history

> models::ContactHistoryResponse get_contact_history(contact_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contact_id** | **String** |  | [required] |

### Return type

[**models::ContactHistoryResponse**](ContactHistoryResponse.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_communications

> Vec<models::CustomerCommunication> list_communications(page, page_size, contact_id, channel, direction, from, to)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**contact_id** | Option<**String**> | Filter history to a single contact. |  |
**channel** | Option<**String**> |  |  |
**direction** | Option<**String**> |  |  |
**from** | Option<**chrono::NaiveDate**> | Only include communications after this ISO date (inclusive). |  |
**to** | Option<**chrono::NaiveDate**> | Only include communications before this ISO date (inclusive). |  |

### Return type

[**Vec<models::CustomerCommunication>**](CustomerCommunication.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_communication

> models::CustomerCommunication update_communication(communication_id, customer_communication_update)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**communication_id** | **String** |  | [required] |
**customer_communication_update** | [**CustomerCommunicationUpdate**](CustomerCommunicationUpdate.md) |  | [required] |

### Return type

[**models::CustomerCommunication**](CustomerCommunication.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

