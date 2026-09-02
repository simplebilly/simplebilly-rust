# JobPostingUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**currency** | Option<**String**> |  | [optional]
**department** | Option<**String**> |  | [optional]
**description** | Option<**String**> | What the job is; markdown/HTML. | [optional]
**employment_type** | Option<[**models::EmploymentType**](EmploymentType.md)> | full_time | part_time | contract | internship | temporary | [optional]
**location** | Option<**String**> |  | [optional]
**remote** | Option<**bool**> |  | [optional]
**required_skills** | Option<**serde_json::Value**> | List of required skill names (JSON array of strings). | [optional]
**requirements** | Option<**String**> | Structured profile of the required candidate (skills, experience). | [optional]
**salary_max** | Option<**i32**> |  | [optional]
**salary_min** | Option<**i32**> |  | [optional]
**status** | Option<[**models::JobPostingStatus**](JobPostingStatus.md)> | draft | published | closed | [optional]
**title** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


