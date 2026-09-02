# AttachmentVersion

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**attachment_id** | **uuid::Uuid** | Parent attachment whose history this row records. | 
**file_name** | **String** | Storage key of this version's bytes. | 
**file_size** | Option<**i64**> |  | [optional]
**mime_type** | Option<**String**> |  | [optional]
**original_name** | Option<**String**> |  | [optional]
**sha256_hash** | Option<**String**> |  | [optional]
**uploaded_by** | Option<**uuid::Uuid**> |  | [optional]
**version_number** | **i32** | 1-based; ascending per attachment in upload order. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


