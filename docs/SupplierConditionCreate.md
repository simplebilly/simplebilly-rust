# SupplierConditionCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**currency** | **String** | Currency for the minimum order value. | 
**delivery_terms** | Option<**String**> | Incoterms, e.g. \"EXW\", \"DAP\". | [optional]
**early_payment_discount_percent** | Option<**String**> | Early-payment discount percentage (Skonto), e.g. 2.0. | [optional]
**is_default** | Option<**bool**> | Is this the default condition for the supplier? | [optional]
**minimum_order_value** | Option<**String**> | Minimum order value required for this supplier. | [optional]
**notes** | Option<**String**> |  | [optional]
**payment_due_days** | Option<**i32**> | Number of days within which payment is due. | [optional]
**payment_terms** | Option<**String**> | Payment terms, e.g. \"14 Tage, 2% Skonto\". | [optional]
**supplier_contact_id** | **String** | The supplier this condition applies to (`contact_id`). References the supplier entity. | 
**supplier_name** | Option<**String**> | The name of the supplier, denormalized for easy listing. | [optional]
**volume_discount_tiers** | Option<**serde_json::Value**> | Tiered discounts: JSON array of `{min_quantity, discount_percent}`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


