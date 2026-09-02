# Job

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**attempts** | Option<**i32**> |  | [optional]
**job_type** | **String** | Discriminator the worker dispatches on (e.g. \"webhook.deliver\"). | 
**max_attempts** | **i32** |  | 
**payload** | Option<**serde_json::Value**> |  | [optional]
**run_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Earliest execution time; None = run now. | [optional]
**status** | [**models::JobStatus**](JobStatus.md) | pending | running | done | failed | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


