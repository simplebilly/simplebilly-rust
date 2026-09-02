# Coupon

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | **String** |  | 
**description** | Option<**String**> |  | [optional]
**discount_type** | [**models::DiscountType**](DiscountType.md) |  | 
**discount_value** | **String** |  | 
**expires_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**is_active** | Option<**bool**> |  | [optional]
**is_combineable** | Option<**bool**> |  | [optional]
**max_discount_amount** | Option<**String**> |  | [optional]
**max_uses** | Option<**i32**> |  | [optional]
**max_uses_per_customer** | Option<**i32**> |  | [optional]
**min_order_amount** | Option<**String**> |  | [optional]
**product_ids** | Option<**serde_json::Value**> |  | [optional]
**starts_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


