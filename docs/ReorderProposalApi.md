# \ReorderProposalApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**apply_reorder_proposal**](ReorderProposalApi.md#apply_reorder_proposal) | **POST** /api/v1/reorder-proposals/apply | Convert a reorder proposal into a draft purchase order.
[**get_reorder_proposal**](ReorderProposalApi.md#get_reorder_proposal) | **GET** /api/v1/reorder-proposals | 



## apply_reorder_proposal

> serde_json::Value apply_reorder_proposal(configured_only, warehouse_id)
Convert a reorder proposal into a draft purchase order.

Returns the created purchase order id. Suggested line items are generated with the current reorder quantity per product.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**configured_only** | Option<**bool**> | Only include products with a reorder point configured (`min_stock`). |  |
**warehouse_id** | Option<**String**> | Limit to a single warehouse id. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_reorder_proposal

> models::ReorderProposalResponse get_reorder_proposal(configured_only, warehouse_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**configured_only** | Option<**bool**> | Only include products with a reorder point configured (`min_stock`). |  |
**warehouse_id** | Option<**String**> | Limit to a single warehouse id. |  |

### Return type

[**models::ReorderProposalResponse**](ReorderProposalResponse.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

