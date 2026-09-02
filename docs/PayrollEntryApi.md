# PayrollEntryApi

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**av_employee** | **String** |  | 
**av_employer** | **String** |  | 
**church_tax_amount** | **String** |  | 
**employee** | Option<[**models::Employee**](Employee.md)> |  | [optional]
**employee_id** | **uuid::Uuid** |  | 
**entry_id** | **uuid::Uuid** |  | 
**extra_payment_reason** | Option<**String**> |  | [optional]
**extra_payments** | **String** |  | 
**gross_salary** | **String** |  | 
**kv_employee** | **String** |  | 
**kv_employer** | **String** |  | 
**lohnsteuer** | **String** |  | 
**net_salary** | **String** |  | 
**notes** | Option<**String**> |  | [optional]
**pv_employee** | **String** |  | 
**pv_employer** | **String** |  | 
**run_id** | **uuid::Uuid** |  | 
**rv_employee** | **String** |  | 
**rv_employer** | **String** |  | 
**sick_days** | **i32** |  | 
**soli** | **String** |  | 
**status** | [**models::PayrollRunStatus**](PayrollRunStatus.md) |  | 
**total_deductions** | **String** |  | 
**total_employer_cost** | **String** |  | 
**vacation_days_used** | **i32** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


