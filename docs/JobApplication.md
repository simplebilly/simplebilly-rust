# JobApplication

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cv_file** | Option<**String**> | Relative path of the stored CV file under the upload dir. | [optional]
**cv_text** | Option<**String**> | Extracted CV text, used for match-scoring. | [optional]
**email** | Option<**String**> |  | [optional]
**match_reason** | Option<**String**> |  | [optional]
**match_score** | Option<**i32**> | 0-100 LLM match score against the posting's required profile. | [optional]
**name** | Option<**String**> |  | [optional]
**phone** | Option<**String**> |  | [optional]
**posting_id** | Option<**uuid::Uuid**> | References the job_posting entity. | [optional]
**source** | **String** | website | email | board | 
**status** | [**models::ApplicationStatus**](ApplicationStatus.md) | new | reviewing | interview | hired | rejected | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


