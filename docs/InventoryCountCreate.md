# InventoryCountCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**count_date** | **chrono::NaiveDate** |  | 
**count_number** | **String** |  | 
**line_items** | Option<**serde_json::Value**> | JSON array of `{product_id, name, sku, expected_quantity, counted_quantity, bin_location?, batch_number?, variance}`. | 
**notes** | Option<**String**> |  | [optional]
**status** | [**models::InventoryCountStatus**](InventoryCountStatus.md) | One of: draft | counting | reviewed | posted | 
**warehouse_id** | **String** | References the warehouse entity. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


