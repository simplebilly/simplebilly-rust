# \SilentPartnerApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_silent_partner**](SilentPartnerApi.md#create_silent_partner) | **POST** /api/v1/silent-partners | 
[**delete_silent_partner**](SilentPartnerApi.md#delete_silent_partner) | **DELETE** /api/v1/silent-partners/{id} | 
[**get_silent_partner**](SilentPartnerApi.md#get_silent_partner) | **GET** /api/v1/silent-partners/{id} | 
[**get_silent_partners**](SilentPartnerApi.md#get_silent_partners) | **GET** /api/v1/silent-partners/ | 
[**update_silent_partner**](SilentPartnerApi.md#update_silent_partner) | **PUT** /api/v1/silent-partners/{id} | 



## create_silent_partner

> models::SilentPartner create_silent_partner(silent_partner_create)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**silent_partner_create** | [**SilentPartnerCreate**](SilentPartnerCreate.md) |  | [required] |

### Return type

[**models::SilentPartner**](SilentPartner.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_silent_partner

> delete_silent_partner(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_silent_partner

> models::SilentPartner get_silent_partner(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::SilentPartner**](SilentPartner.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_silent_partners

> Vec<models::SilentPartner> get_silent_partners(page, page_size, search, include_deleted)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**search** | Option<**String**> |  |  |
**include_deleted** | Option<**bool**> | Soft-delete entities: set true to include rows with `deleted_at` set. |  |

### Return type

[**Vec<models::SilentPartner>**](SilentPartner.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_silent_partner

> models::SilentPartner update_silent_partner(id, silent_partner_update)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**silent_partner_update** | [**SilentPartnerUpdate**](SilentPartnerUpdate.md) |  | [required] |

### Return type

[**models::SilentPartner**](SilentPartner.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

