# BomUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**components** | Option<**serde_json::Value**> | JSON array of `{product_id, name, quantity, unit, scrap_rate}`. | [optional]
**description** | Option<**String**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**output_quantity** | Option<**i64**> | Output quantity per production run (defaults to 1). | [optional]
**product_id** | Option<**uuid::Uuid**> | The finished product this BOM produces. References the product entity. | [optional]
**status** | Option<[**models::BomStatus**](BomStatus.md)> | One of: draft | active | archived | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


