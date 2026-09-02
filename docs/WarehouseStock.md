# WarehouseStock

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**batch_number** | Option<**String**> | Batch/lot number (Chargennummer) — `None` for non-batched goods. | [optional]
**bin_location** | Option<**String**> |  | [optional]
**expiry_date** | Option<**chrono::NaiveDate**> | Expiry date for batch-tracked goods. | [optional]
**product_id** | **uuid::Uuid** |  | 
**quantity** | **i64** |  | 
**serial_numbers** | Option<**serde_json::Value**> | JSON array of serial numbers (Seriennummern) in this stock row. | [optional]
**warehouse_id** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


