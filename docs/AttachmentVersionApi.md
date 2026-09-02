# \AttachmentVersionApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_attachment_version**](AttachmentVersionApi.md#create_attachment_version) | **POST** /api/v1/attachments/{attachment_id}/versions | 
[**list_attachment_versions**](AttachmentVersionApi.md#list_attachment_versions) | **GET** /api/v1/attachments/{attachment_id}/versions | 
[**restore_attachment_version**](AttachmentVersionApi.md#restore_attachment_version) | **POST** /api/v1/attachments/{attachment_id}/versions/{version_id}/restore | 



## create_attachment_version

> models::AttachmentVersion create_attachment_version(attachment_id, new_version_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**attachment_id** | **uuid::Uuid** |  | [required] |
**new_version_request** | [**NewVersionRequest**](NewVersionRequest.md) |  | [required] |

### Return type

[**models::AttachmentVersion**](AttachmentVersion.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_attachment_versions

> Vec<models::AttachmentVersion> list_attachment_versions(attachment_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**attachment_id** | **uuid::Uuid** |  | [required] |

### Return type

[**Vec<models::AttachmentVersion>**](AttachmentVersion.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## restore_attachment_version

> models::Attachment restore_attachment_version(attachment_id, version_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**attachment_id** | **uuid::Uuid** |  | [required] |
**version_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::Attachment**](Attachment.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

