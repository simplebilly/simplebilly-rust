# TicketMessage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**author_email** | Option<**String**> |  | [optional]
**author_name** | Option<**String**> |  | [optional]
**body** | **String** |  | 
**body_html** | Option<**String**> |  | [optional]
**channel_id** | Option<**uuid::Uuid**> |  | [optional]
**created_at** | **chrono::DateTime<chrono::FixedOffset>** |  | 
**direction** | [**models::MessageDirection**](MessageDirection.md) |  | 
**external_id** | Option<**String**> |  | [optional]
**is_internal** | **bool** |  | 
**message_type** | [**models::MessageType**](MessageType.md) |  | 
**metadata** | Option<**serde_json::Value**> |  | 
**tenant_id** | **uuid::Uuid** |  | 
**ticket_id** | **uuid::Uuid** | References the ticket entity. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


