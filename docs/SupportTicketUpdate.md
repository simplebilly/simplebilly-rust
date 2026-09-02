# SupportTicketUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**assigned_to** | Option<**uuid::Uuid**> |  | [optional]
**channel_id** | Option<**uuid::Uuid**> |  | [optional]
**channel_type** | Option<[**models::SupportChannelType**](SupportChannelType.md)> |  | [optional]
**closed_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**created_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**customer_email** | Option<**String**> |  | [optional]
**customer_id** | Option<**String**> | References the customer entity. | [optional]
**customer_name** | Option<**String**> |  | [optional]
**external_id** | Option<**String**> |  | [optional]
**first_message_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**last_message_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**lead_id** | Option<**uuid::Uuid**> | References the lead entity. | [optional]
**message_count** | Option<**i32**> |  | [optional]
**order_ref** | Option<**String**> |  | [optional]
**priority** | Option<[**models::TicketPriority**](TicketPriority.md)> |  | [optional]
**resolution** | Option<**String**> |  | [optional]
**status** | Option<[**models::SupportTicketStatus**](SupportTicketStatus.md)> |  | [optional]
**subject** | Option<**String**> |  | [optional]
**tags** | Option<**serde_json::Value**> |  | [optional]
**tenant_id** | Option<**uuid::Uuid**> |  | [optional]
**updated_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


