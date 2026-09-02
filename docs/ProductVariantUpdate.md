# ProductVariantUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**barcode** | Option<**String**> |  | [optional]
**image_link** | Option<**String**> |  | [optional]
**is_active** | Option<**bool**> |  | [optional]
**name** | Option<**String**> | Human-readable variant label, e.g. \"Red / M\". | [optional]
**option_values** | Option<**serde_json::Value**> | Option name → value map, e.g. `{\"Color\": \"Red\", \"Size\": \"M\"}`. | [optional]
**price** | Option<**String**> | Explicit override price for this variant (takes precedence over parent price + delta). | [optional]
**price_delta** | Option<**String**> | Price adjustment relative to the parent product's `default_price`. | [optional]
**product_id** | Option<**uuid::Uuid**> | The parent product this variant belongs to. References the product entity. | [optional]
**sku** | Option<**String**> | Variant-specific SKU (must be unique per tenant). | [optional]
**stock_quantity** | Option<**i64**> | Variant-level stock (optional — may be tracked on the parent only). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


