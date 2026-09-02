# \GroupFigureApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_group_figure**](GroupFigureApi.md#create_group_figure) | **POST** /api/v1/group-figures | 
[**delete_group_figure**](GroupFigureApi.md#delete_group_figure) | **DELETE** /api/v1/group-figures/{year} | 
[**get_group_figure**](GroupFigureApi.md#get_group_figure) | **GET** /api/v1/group-figures/{year} | 
[**get_group_figures**](GroupFigureApi.md#get_group_figures) | **GET** /api/v1/group-figures/ | 
[**update_group_figure**](GroupFigureApi.md#update_group_figure) | **PUT** /api/v1/group-figures/{year} | 



## create_group_figure

> models::GroupFigure create_group_figure(group_figure_create)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_figure_create** | [**GroupFigureCreate**](GroupFigureCreate.md) |  | [required] |

### Return type

[**models::GroupFigure**](GroupFigure.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_group_figure

> delete_group_figure(year)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_figure

> models::GroupFigure get_group_figure(year)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | **i32** |  | [required] |

### Return type

[**models::GroupFigure**](GroupFigure.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_figures

> Vec<models::GroupFigure> get_group_figures(page, page_size, search, include_deleted)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**search** | Option<**String**> |  |  |
**include_deleted** | Option<**bool**> | Soft-delete entities: set true to include rows with `deleted_at` set. |  |

### Return type

[**Vec<models::GroupFigure>**](GroupFigure.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_group_figure

> models::GroupFigure update_group_figure(year, group_figure_update)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | **i32** |  | [required] |
**group_figure_update** | [**GroupFigureUpdate**](GroupFigureUpdate.md) |  | [required] |

### Return type

[**models::GroupFigure**](GroupFigure.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

