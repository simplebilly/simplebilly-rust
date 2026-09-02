# InventoryCountUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**count_date** | Option<**chrono::NaiveDate**> |  | [optional]
**count_number** | Option<**String**> |  | [optional]
**line_items** | Option<**serde_json::Value**> | JSON array of `{product_id, name, sku, expected_quantity, counted_quantity, bin_location?, batch_number?, variance}`. | [optional]
**notes** | Option<**String**> |  | [optional]
**status** | Option<[**models::InventoryCountStatus**](InventoryCountStatus.md)> | One of: draft | counting | reviewed | posted | [optional]
**warehouse_id** | Option<**String**> | References the warehouse entity. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


