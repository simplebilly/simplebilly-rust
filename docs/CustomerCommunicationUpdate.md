# CustomerCommunicationUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**body** | Option<**String**> | The message body, call summary or note text. | [optional]
**channel** | Option<[**models::CommunicationChannel**](CommunicationChannel.md)> |  | [optional]
**contact_id** | Option<**String**> | The contact (customer/supplier) this communication belongs to. References the contact entity. | [optional]
**counterparty** | Option<**String**> | Email/phone of the counterparty, if applicable. | [optional]
**direction** | Option<[**models::CommunicationDirection**](CommunicationDirection.md)> |  | [optional]
**occurred_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | When the communication happened (defaults to now on create). | [optional]
**subject** | Option<**String**> |  | [optional]
**tags** | Option<**serde_json::Value**> | Free-form tags, e.g. `[\"follow-up-required\"]`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


