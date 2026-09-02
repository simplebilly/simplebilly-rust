# \BudgetsApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**budgets_api**](BudgetsApi.md#budgets_api) | **GET** /api/v1/bookkeeping/budgets | 
[**upsert_budget_goal_api**](BudgetsApi.md#upsert_budget_goal_api) | **PUT** /api/v1/bookkeeping/budgets/goals/{category} | 



## budgets_api

> models::BudgetErgebnis budgets_api(year, month)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | **i32** |  | [required] |
**month** | **i32** |  | [required] |

### Return type

[**models::BudgetErgebnis**](BudgetErgebnis.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upsert_budget_goal_api

> models::Budget upsert_budget_goal_api(category, budget_goal_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**category** | **String** |  | [required] |
**budget_goal_request** | [**BudgetGoalRequest**](BudgetGoalRequest.md) |  | [required] |

### Return type

[**models::Budget**](Budget.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

