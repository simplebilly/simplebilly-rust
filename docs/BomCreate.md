# BomCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**components** | Option<**serde_json::Value**> | JSON array of `{product_id, name, quantity, unit, scrap_rate}`. | [optional]
**description** | Option<**String**> |  | [optional]
**name** | **String** |  | 
**output_quantity** | Option<**i64**> | Output quantity per production run (defaults to 1). | [optional]
**product_id** | **uuid::Uuid** | The finished product this BOM produces. References the product entity. | 
**status** | Option<[**models::BomStatus**](BomStatus.md)> | One of: draft | active | archived | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


