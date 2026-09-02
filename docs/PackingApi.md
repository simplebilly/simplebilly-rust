# \PackingApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**complete_packing**](PackingApi.md#complete_packing) | **POST** /api/v1/packing/{order_number}/complete | Mark packing as complete and transition order to shipped
[**get_packing_queue**](PackingApi.md#get_packing_queue) | **GET** /api/v1/packing/queue | Get the packing queue - orders ready for packing
[**print_delivery_note**](PackingApi.md#print_delivery_note) | **POST** /api/v1/packing/{order_number}/print-delivery-note | Print delivery note (Lieferschein) for an order
[**print_label**](PackingApi.md#print_label) | **POST** /api/v1/packing/{order_number}/print-label | Print shipping label for an order
[**record_packing_video**](PackingApi.md#record_packing_video) | **POST** /api/v1/packing/{order_number}/record-video | Record video of packing process



## complete_packing

> models::PackingCompleteResponse complete_packing(order_number, packing_complete_request)
Mark packing as complete and transition order to shipped

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_number** | **String** |  | [required] |
**packing_complete_request** | [**PackingCompleteRequest**](PackingCompleteRequest.md) |  | [required] |

### Return type

[**models::PackingCompleteResponse**](PackingCompleteResponse.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_packing_queue

> models::PackingQueue get_packing_queue(page, page_size, search)
Get the packing queue - orders ready for packing

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**search** | Option<**String**> |  |  |

### Return type

[**models::PackingQueue**](PackingQueue.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## print_delivery_note

> models::PrintDeliveryNoteResponse print_delivery_note(order_number)
Print delivery note (Lieferschein) for an order

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_number** | **String** |  | [required] |

### Return type

[**models::PrintDeliveryNoteResponse**](PrintDeliveryNoteResponse.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## print_label

> models::PrintLabelResponse print_label(order_number)
Print shipping label for an order

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_number** | **String** |  | [required] |

### Return type

[**models::PrintLabelResponse**](PrintLabelResponse.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## record_packing_video

> models::PackingVideoResponse record_packing_video(order_number, body)
Record video of packing process

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_number** | **String** |  | [required] |
**body** | Option<**serde_json::Value**> |  | [required] |

### Return type

[**models::PackingVideoResponse**](PackingVideoResponse.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

