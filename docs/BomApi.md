# \BomApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_bom**](BomApi.md#create_bom) | **POST** /api/v1/boms | 
[**delete_bom**](BomApi.md#delete_bom) | **DELETE** /api/v1/boms/{bom_id} | 
[**get_bom**](BomApi.md#get_bom) | **GET** /api/v1/boms/{bom_id} | 
[**list_boms**](BomApi.md#list_boms) | **GET** /api/v1/boms/ | 
[**update_bom**](BomApi.md#update_bom) | **PUT** /api/v1/boms/{bom_id} | 



## create_bom

> models::Bom create_bom(bom_create)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bom_create** | [**BomCreate**](BomCreate.md) |  | [required] |

### Return type

[**models::Bom**](Bom.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_bom

> delete_bom(bom_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bom_id** | **uuid::Uuid** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bom

> models::Bom get_bom(bom_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bom_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::Bom**](Bom.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_boms

> Vec<models::Bom> list_boms(page, page_size, search, product_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**search** | Option<**String**> |  |  |
**product_id** | Option<**uuid::Uuid**> | Filter by finished product id. |  |

### Return type

[**Vec<models::Bom>**](Bom.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_bom

> models::Bom update_bom(bom_id, bom_update)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bom_id** | **uuid::Uuid** |  | [required] |
**bom_update** | [**BomUpdate**](BomUpdate.md) |  | [required] |

### Return type

[**models::Bom**](Bom.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

