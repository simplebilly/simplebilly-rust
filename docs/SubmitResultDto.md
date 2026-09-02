# SubmitResultDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**answers** | **Vec<i32>** | Selected answer indices (required for scored builtin trainings). | 
**assignment_id** | Option<**uuid::Uuid**> |  | [optional]
**score** | **i32** | Score 0–100. Only trusted for plugin trainings without server-side scoring; builtin trainings are always re-scored from `answers`. | 
**training_code** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


