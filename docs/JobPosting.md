# JobPosting

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**currency** | Option<**String**> |  | [optional]
**department** | Option<**String**> |  | [optional]
**description** | **String** | What the job is; markdown/HTML. | 
**employment_type** | Option<[**models::EmploymentType**](EmploymentType.md)> | full_time | part_time | contract | internship | temporary | [optional]
**location** | Option<**String**> |  | [optional]
**remote** | **bool** |  | 
**required_skills** | Option<**serde_json::Value**> | List of required skill names (JSON array of strings). | 
**requirements** | Option<**String**> | Structured profile of the required candidate (skills, experience). | [optional]
**salary_max** | Option<**i32**> |  | [optional]
**salary_min** | Option<**i32**> |  | [optional]
**status** | [**models::JobPostingStatus**](JobPostingStatus.md) | draft | published | closed | 
**title** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


