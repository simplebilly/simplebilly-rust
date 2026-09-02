# EmailTemplateCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**body** | **String** | E-mail body with optional placeholders. | 
**name** | **String** | Human-readable template name, e.g. \"Follow-up after quote\". | 
**status** | [**models::EmailTemplateStatus**](EmailTemplateStatus.md) | One of: active | inactive | 
**subject** | **String** | E-mail subject line with optional placeholders. | 
**variables** | Option<**serde_json::Value**> | Placeholders used by this template, e.g. `[\"contact.first_name\"]`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


