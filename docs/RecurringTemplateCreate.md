# RecurringTemplateCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**end_date** | Option<**chrono::NaiveDate**> |  | [optional]
**execution_interval** | **String** |  | 
**execution_status** | [**models::ExecutionStatus**](ExecutionStatus.md) |  | 
**finalize** | Option<**bool**> |  | [optional]
**last_executed_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**name** | **String** |  | 
**next_execution_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**start_date** | **chrono::NaiveDate** |  | 
**template_type** | [**models::RecurringTemplateType**](RecurringTemplateType.md) |  | 
**voucher_data** | Option<**serde_json::Value**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


