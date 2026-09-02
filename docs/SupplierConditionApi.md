# \SupplierConditionApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_supplier_condition**](SupplierConditionApi.md#create_supplier_condition) | **POST** /api/v1/supplier-conditions | 
[**delete_supplier_condition**](SupplierConditionApi.md#delete_supplier_condition) | **DELETE** /api/v1/supplier-conditions/{supplier_condition_id} | 
[**get_supplier_condition**](SupplierConditionApi.md#get_supplier_condition) | **GET** /api/v1/supplier-conditions/{supplier_condition_id} | 
[**list_supplier_conditions**](SupplierConditionApi.md#list_supplier_conditions) | **GET** /api/v1/supplier-conditions/ | 
[**update_supplier_condition**](SupplierConditionApi.md#update_supplier_condition) | **PUT** /api/v1/supplier-conditions/{supplier_condition_id} | 



## create_supplier_condition

> models::SupplierCondition create_supplier_condition(supplier_condition_create)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**supplier_condition_create** | [**SupplierConditionCreate**](SupplierConditionCreate.md) |  | [required] |

### Return type

[**models::SupplierCondition**](SupplierCondition.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_supplier_condition

> delete_supplier_condition(supplier_condition_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**supplier_condition_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_supplier_condition

> models::SupplierCondition get_supplier_condition(supplier_condition_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**supplier_condition_id** | **String** |  | [required] |

### Return type

[**models::SupplierCondition**](SupplierCondition.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_supplier_conditions

> Vec<models::SupplierCondition> list_supplier_conditions(page, page_size, supplier_contact_id, search)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**supplier_contact_id** | Option<**String**> |  |  |
**search** | Option<**String**> |  |  |

### Return type

[**Vec<models::SupplierCondition>**](SupplierCondition.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_supplier_condition

> models::SupplierCondition update_supplier_condition(supplier_condition_id, supplier_condition_update)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**supplier_condition_id** | **String** |  | [required] |
**supplier_condition_update** | [**SupplierConditionUpdate**](SupplierConditionUpdate.md) |  | [required] |

### Return type

[**models::SupplierCondition**](SupplierCondition.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

