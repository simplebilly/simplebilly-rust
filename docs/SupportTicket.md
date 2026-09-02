# SupportTicket

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**assigned_to** | Option<**uuid::Uuid**> |  | [optional]
**channel_id** | Option<**uuid::Uuid**> |  | [optional]
**channel_type** | Option<[**models::SupportChannelType**](SupportChannelType.md)> |  | [optional]
**closed_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**created_at** | **chrono::DateTime<chrono::FixedOffset>** |  | 
**customer_email** | Option<**String**> |  | [optional]
**customer_id** | Option<**String**> | References the customer entity. | [optional]
**customer_name** | Option<**String**> |  | [optional]
**external_id** | Option<**String**> |  | [optional]
**first_message_at** | **chrono::DateTime<chrono::FixedOffset>** |  | 
**last_message_at** | **chrono::DateTime<chrono::FixedOffset>** |  | 
**lead_id** | Option<**uuid::Uuid**> | References the lead entity. | [optional]
**message_count** | **i32** |  | 
**order_ref** | Option<**String**> |  | [optional]
**priority** | [**models::TicketPriority**](TicketPriority.md) |  | 
**resolution** | Option<**String**> |  | [optional]
**status** | [**models::SupportTicketStatus**](SupportTicketStatus.md) |  | 
**subject** | **String** |  | 
**tags** | Option<**serde_json::Value**> |  | 
**tenant_id** | **uuid::Uuid** |  | 
**updated_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


