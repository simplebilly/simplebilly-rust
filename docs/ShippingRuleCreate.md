# ShippingRuleCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**carrier** | Option<**String**> | Provider that auto-filled this rule (e.g. \"ups\"), if any. | [optional]
**country** | Option<[**models::CountryCode**](CountryCode.md)> | None = applies to all countries. | [optional]
**delivery_time** | Option<**String**> | Delivery time text, e.g. \"1-3\". | [optional]
**is_active** | Option<**bool**> |  | [optional]
**max_weight_kg** | Option<**f64**> |  | [optional]
**min_weight_kg** | Option<**f64**> |  | [optional]
**name** | **String** | Delivery-method label, e.g. \"Standardversand\". | 
**notes** | Option<**String**> |  | [optional]
**price** | **String** | Shipping cost in the shop's currency. | 
**priority** | Option<**i32**> | Lower wins when multiple rules match. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


