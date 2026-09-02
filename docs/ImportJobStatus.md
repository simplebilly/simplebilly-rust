# ImportJobStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**error** | Option<**String**> | Set only when the job failed. | [optional]
**job_id** | **String** |  | 
**processed** | **i64** |  | 
**progress** | **i32** | 0–100 | 
**provider** | Option<**String**> | Which competitor the import came from (lexoffice | billbee); the frontend uses it to label the job. Absent for legacy jobs. | [optional]
**stage** | **String** | queued | fetching | downloading | importing | done | 
**status** | **String** | pending | running | done | failed | 
**total** | **i64** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


