# \DeliveryNoteApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_delivery_note**](DeliveryNoteApi.md#create_delivery_note) | **POST** /api/v1/delivery-notes | 
[**delete_delivery_note**](DeliveryNoteApi.md#delete_delivery_note) | **DELETE** /api/v1/delivery-notes/{delivery_note_id} | 
[**deliverynote_restore**](DeliveryNoteApi.md#deliverynote_restore) | **POST** /api/v1/delivery-notes/{delivery_note_id}/restore | 
[**download_delivery_note_pdf**](DeliveryNoteApi.md#download_delivery_note_pdf) | **GET** /api/v1/delivery-notes/{delivery_note_id}/pdf | 
[**get_delivery_note**](DeliveryNoteApi.md#get_delivery_note) | **GET** /api/v1/delivery-notes/{delivery_note_id} | 
[**list_delivery_notes**](DeliveryNoteApi.md#list_delivery_notes) | **GET** /api/v1/delivery-notes/ | 
[**pursue_delivery_note**](DeliveryNoteApi.md#pursue_delivery_note) | **POST** /api/v1/delivery-notes/{delivery_note_id}/pursue | 



## create_delivery_note

> models::DeliveryNote create_delivery_note(delivery_note_create)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delivery_note_create** | [**DeliveryNoteCreate**](DeliveryNoteCreate.md) |  | [required] |

### Return type

[**models::DeliveryNote**](DeliveryNote.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_delivery_note

> delete_delivery_note(delivery_note_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delivery_note_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deliverynote_restore

> models::DeliveryNote deliverynote_restore(delivery_note_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delivery_note_id** | **String** |  | [required] |

### Return type

[**models::DeliveryNote**](DeliveryNote.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## download_delivery_note_pdf

> download_delivery_note_pdf(delivery_note_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delivery_note_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/pdf, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_delivery_note

> models::DeliveryNote get_delivery_note(delivery_note_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delivery_note_id** | **String** |  | [required] |

### Return type

[**models::DeliveryNote**](DeliveryNote.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_delivery_notes

> Vec<models::DeliveryNote> list_delivery_notes(page, page_size, search, include_deleted)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**search** | Option<**String**> |  |  |
**include_deleted** | Option<**bool**> | Soft-delete entities: set true to include rows with `deleted_at` set. |  |

### Return type

[**Vec<models::DeliveryNote>**](DeliveryNote.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pursue_delivery_note

> models::Invoice pursue_delivery_note(delivery_note_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delivery_note_id** | **String** |  | [required] |

### Return type

[**models::Invoice**](Invoice.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

