# ReturnOrder

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**customer_contact_id** | Option<**String**> | References the contact entity. | [optional]
**customer_name** | Option<**String**> |  | [optional]
**line_items** | Option<**serde_json::Value**> | JSON array of `{product_id, name, quantity, condition, restock, batch_number?}`. | [optional]
**notes** | Option<**String**> |  | [optional]
**order_id** | Option<**String**> | References the order entity. | [optional]
**order_number** | Option<**String**> |  | [optional]
**return_number** | **String** |  | 
**return_reason** | Option<**String**> |  | [optional]
**status** | [**models::ReturnOrderStatus**](ReturnOrderStatus.md) | One of: requested | received | inspected | restocked | closed | 
**warehouse_id** | Option<**String**> | Warehouse into which restockable items are returned. References the warehouse entity. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


