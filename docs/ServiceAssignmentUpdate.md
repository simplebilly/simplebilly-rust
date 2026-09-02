# ServiceAssignmentUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**employee_id** | Option<**uuid::Uuid**> | References the employees entity. | [optional]
**job_id** | Option<**uuid::Uuid**> | References the service_jobs entity. | [optional]
**notes** | Option<**String**> |  | [optional]
**scheduled_date** | Option<**chrono::NaiveDate**> | Work day the assignment is scheduled for. | [optional]
**scheduled_end** | Option<**String**> | Planned end time of the assignment. | [optional]
**scheduled_start** | Option<**String**> | Planned start time of the assignment. | [optional]
**status** | Option<[**models::ServiceAssignmentStatus**](ServiceAssignmentStatus.md)> | Assignment lifecycle status: \"planned\", \"confirmed\", \"en_route\", \"in_progress\", \"completed\" or \"cancelled\". | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


