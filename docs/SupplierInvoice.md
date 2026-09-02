# SupplierInvoice

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**currency** | Option<**String**> |  | [optional]
**goods_receipt_id** | Option<**String**> | References the goods receipt entity. | [optional]
**invoice_date** | **chrono::NaiveDate** |  | 
**invoice_number** | **String** |  | 
**line_items** | Option<**serde_json::Value**> | JSON array of `{product_id, name, quantity, unitPriceNet, taxRate}`. | 
**notes** | Option<**String**> |  | [optional]
**purchase_order_id** | Option<**String**> | References the purchase order entity. | [optional]
**status** | [**models::SupplierInvoiceStatus**](SupplierInvoiceStatus.md) | One of: draft | matched | has_variances | posted | cancelled | 
**supplier_contact_id** | Option<**String**> | References the supplier entity. | [optional]
**supplier_name** | Option<**String**> |  | [optional]
**total_gross_amount** | Option<**String**> |  | [optional]
**total_net_amount** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


