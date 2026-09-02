# OrderUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**audit_log** | Option<**serde_json::Value**> |  | [optional]
**currency** | Option<**String**> |  | [optional]
**customer_id** | Option<**String**> | References the customer entity. | [optional]
**external_reference** | Option<**String**> |  | [optional]
**invoice_address** | Option<**serde_json::Value**> |  | [optional]
**items** | Option<**serde_json::Value**> |  | [optional]
**language** | Option<[**models::LanguageCode**](LanguageCode.md)> |  | [optional]
**order_status** | Option<[**models::OrderStatus**](OrderStatus.md)> |  | [optional]
**payment_method** | Option<[**models::PaymentMethod**](PaymentMethod.md)> |  | [optional]
**shipping_address** | Option<**serde_json::Value**> |  | [optional]
**shipping_cost** | Option<**String**> |  | [optional]
**shipping_method** | Option<**String**> |  | [optional]
**shipping_weight** | Option<**String**> |  | [optional]
**tags** | Option<**Vec<String>**> |  | [optional]
**total_cost** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


