# \ProposeAssignmentsApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**propose_assignments_api**](ProposeAssignmentsApi.md#propose_assignments_api) | **GET** /api/v1/bookkeeping/propose-assignments | 



## propose_assignments_api

> Vec<models::ProposedAssignment> propose_assignments_api(min_confidence, customer_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**min_confidence** | Option<**f64**> |  |  |
**customer_id** | Option<**String**> |  |  |

### Return type

[**Vec<models::ProposedAssignment>**](ProposedAssignment.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

