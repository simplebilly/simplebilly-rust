# Order

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**audit_log** | Option<**serde_json::Value**> |  | [optional]
**currency** | **String** |  | 
**customer_id** | **String** | References the customer entity. | 
**external_reference** | Option<**String**> |  | [optional]
**invoice_address** | Option<**serde_json::Value**> |  | [optional]
**items** | Option<**serde_json::Value**> |  | [optional]
**language** | Option<[**models::LanguageCode**](LanguageCode.md)> |  | [optional]
**order_status** | [**models::OrderStatus**](OrderStatus.md) |  | 
**payment_method** | [**models::PaymentMethod**](PaymentMethod.md) |  | 
**shipping_address** | Option<**serde_json::Value**> |  | [optional]
**shipping_cost** | **String** |  | 
**shipping_method** | **String** |  | 
**shipping_weight** | **String** |  | 
**tags** | **Vec<String>** |  | 
**total_cost** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


