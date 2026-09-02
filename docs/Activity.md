# Activity

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**activity_type** | [**models::ActivityType**](ActivityType.md) | One of: call | email | meeting | task | note | 
**assigned_to** | Option<**String**> | User responsible (`employee.employee_id`). | [optional]
**contact_id** | Option<**String**> | Contact this activity belongs to (`contact.contact_id`). References the contact entity. | [optional]
**description** | Option<**String**> |  | [optional]
**due_date** | Option<**chrono::NaiveDate**> | Follow-up / Wiedervorlage date. Open activities with a due date in the past are overdue. | [optional]
**reminder_date** | Option<**chrono::NaiveDate**> | When to remind about the follow-up. | [optional]
**status** | [**models::ActivityStatus**](ActivityStatus.md) | One of: open | done | cancelled | 
**subject** | **String** | Short subject line. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


