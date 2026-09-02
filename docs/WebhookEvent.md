# WebhookEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**attempts** | Option<**i32**> |  | [optional]
**channel** | Option<**String**> | source for inbound, target URL for outbound. | [optional]
**direction** | [**models::WebhookDirection**](WebhookDirection.md) | inbound | outbound | 
**event_type** | **String** |  | 
**last_error** | Option<**String**> |  | [optional]
**payload** | Option<**serde_json::Value**> |  | [optional]
**status** | Option<[**models::WebhookEventStatus**](WebhookEventStatus.md)> | accepted | delivered | failed | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


