# Payment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amount** | Option<**String**> |  | [optional]
**attachment** | Option<**serde_json::Value**> |  | [optional]
**currency** | Option<**String**> |  | [optional]
**customer_id** | Option<**String**> | References the customer entity. | [optional]
**description** | Option<**String**> |  | [optional]
**metadata** | Option<**serde_json::Value**> |  | [optional]
**method** | Option<[**models::PaymentMethod**](PaymentMethod.md)> |  | [optional]
**payment_date** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**reference** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


