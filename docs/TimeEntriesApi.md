# \TimeEntriesApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**clock_in_time_entry**](TimeEntriesApi.md#clock_in_time_entry) | **POST** /api/v1/time-entries | Clock in for the authenticated user (resolved via their employee profile).
[**clock_out_time_entry**](TimeEntriesApi.md#clock_out_time_entry) | **PATCH** /api/v1/time-entries/{id} | Clock out an entry: the entry's owner, or anyone with `time_entries:write`.
[**get_labor_costs**](TimeEntriesApi.md#get_labor_costs) | **GET** /api/v1/labor-costs | Labor-cost report: worked hours aggregated per employee / order / day, valued at the employee's hourly cost rate.
[**list_time_entries**](TimeEntriesApi.md#list_time_entries) | **GET** /api/v1/time-entries | List time entries with optional date-range / active / employee filters.



## clock_in_time_entry

> models::TimeEntryDto clock_in_time_entry(time_entry_clock_in)
Clock in for the authenticated user (resolved via their employee profile).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**time_entry_clock_in** | [**TimeEntryClockIn**](TimeEntryClockIn.md) |  | [required] |

### Return type

[**models::TimeEntryDto**](TimeEntryDto.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clock_out_time_entry

> models::TimeEntryDto clock_out_time_entry(id, time_entry_clock_out)
Clock out an entry: the entry's owner, or anyone with `time_entries:write`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**time_entry_clock_out** | [**TimeEntryClockOut**](TimeEntryClockOut.md) |  | [required] |

### Return type

[**models::TimeEntryDto**](TimeEntryDto.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_labor_costs

> Vec<models::LaborCostRow> get_labor_costs(from, to, group_by)
Labor-cost report: worked hours aggregated per employee / order / day, valued at the employee's hourly cost rate.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from** | **chrono::NaiveDate** |  | [required] |
**to** | **chrono::NaiveDate** |  | [required] |
**group_by** | **String** | One of \"employee\", \"order\" or \"day\". | [required] |

### Return type

[**Vec<models::LaborCostRow>**](LaborCostRow.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_time_entries

> Vec<models::TimeEntryDto> list_time_entries(from, to, active, employee_id)
List time entries with optional date-range / active / employee filters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from** | Option<**chrono::NaiveDate**> |  |  |
**to** | Option<**chrono::NaiveDate**> |  |  |
**active** | Option<**bool**> | Only currently running shifts (clock_in set, clock_out null). |  |
**employee_id** | Option<**uuid::Uuid**> |  |  |

### Return type

[**Vec<models::TimeEntryDto>**](TimeEntryDto.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

