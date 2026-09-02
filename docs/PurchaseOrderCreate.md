# PurchaseOrderCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**currency** | Option<**String**> |  | [optional]
**delivery_address** | Option<**serde_json::Value**> |  | [optional]
**expected_delivery_date** | Option<**chrono::NaiveDate**> |  | [optional]
**line_items** | Option<**serde_json::Value**> | JSON array of `{product_id, name, quantity, unit_price_net, tax_rate, delivery_date}`. | [optional]
**notes** | Option<**String**> |  | [optional]
**order_date** | **chrono::NaiveDate** |  | 
**po_number** | **String** |  | 
**status** | [**models::PurchaseOrderStatus**](PurchaseOrderStatus.md) | One of: draft | ordered | partially_received | received | cancelled | 
**supplier_contact_id** | Option<**String**> | References the supplier entity. | [optional]
**supplier_name** | Option<**String**> |  | [optional]
**total_gross_amount** | Option<**String**> |  | [optional]
**total_net_amount** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


