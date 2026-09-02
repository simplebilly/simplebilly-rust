# PayrollRunApi

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**approved_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**approved_by** | Option<**uuid::Uuid**> |  | [optional]
**created_at** | **chrono::DateTime<chrono::FixedOffset>** |  | 
**entries** | [**Vec<models::PayrollEntryApi>**](PayrollEntryApi.md) |  | 
**month** | **i32** |  | 
**payment_date** | Option<**chrono::NaiveDate**> |  | [optional]
**period_label** | **String** |  | 
**run_id** | **uuid::Uuid** |  | 
**status** | [**models::PayrollRunStatus**](PayrollRunStatus.md) |  | 
**tenant_id** | **uuid::Uuid** |  | 
**total_employee_count** | **i32** |  | 
**total_employer_cost** | **String** |  | 
**total_gross** | **String** |  | 
**total_net** | **String** |  | 
**total_social_security** | **String** |  | 
**total_taxes** | **String** |  | 
**updated_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**year** | **i32** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


