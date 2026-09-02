# ActivityUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**activity_type** | Option<[**models::ActivityType**](ActivityType.md)> | One of: call | email | meeting | task | note | [optional]
**assigned_to** | Option<**String**> | User responsible (`employee.employee_id`). | [optional]
**contact_id** | Option<**String**> | Contact this activity belongs to (`contact.contact_id`). References the contact entity. | [optional]
**description** | Option<**String**> |  | [optional]
**due_date** | Option<**chrono::NaiveDate**> | Follow-up / Wiedervorlage date. Open activities with a due date in the past are overdue. | [optional]
**reminder_date** | Option<**chrono::NaiveDate**> | When to remind about the follow-up. | [optional]
**status** | Option<[**models::ActivityStatus**](ActivityStatus.md)> | One of: open | done | cancelled | [optional]
**subject** | Option<**String**> | Short subject line. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


