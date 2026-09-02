# GoodsReceipt

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**gr_number** | **String** |  | 
**line_items** | Option<**serde_json::Value**> | JSON array of `{product_id, name, quantity, batch_number?, expiry_date?, bin_location?}`. | 
**notes** | Option<**String**> |  | [optional]
**purchase_order_id** | Option<**String**> | References the purchase order entity. | [optional]
**receipt_date** | **chrono::NaiveDate** |  | 
**supplier_contact_id** | Option<**String**> | References the supplier entity. | [optional]
**supplier_name** | Option<**String**> |  | [optional]
**warehouse_id** | **String** | References the warehouse entity. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


