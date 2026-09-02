# StockTransfer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**line_items** | Option<**serde_json::Value**> | JSON array of `{product_id, name, quantity, batch_number?}`. | 
**notes** | Option<**String**> |  | [optional]
**source_warehouse_id** | **String** | References the warehouse entity. | 
**status** | [**models::StockTransferStatus**](StockTransferStatus.md) | One of: draft | completed | cancelled | 
**target_warehouse_id** | **String** | References the warehouse entity. | 
**transfer_date** | **chrono::NaiveDate** |  | 
**transfer_number** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


