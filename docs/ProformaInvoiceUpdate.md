# ProformaInvoiceUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**converted_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**converted_to_invoice_id** | Option<**String**> | Set when the proforma was converted into a real invoice. References the invoice entity. | [optional]
**currency** | Option<[**models::CurrencyCode**](CurrencyCode.md)> |  | [optional]
**customer_id** | Option<**String**> | References the customer entity. | [optional]
**customer_snapshot** | Option<**serde_json::Value**> | Snapshot of the recipient at issue time (address, VAT id, …). | [optional]
**issue_date** | Option<**chrono::NaiveDate**> |  | [optional]
**line_items** | Option<**serde_json::Value**> |  | [optional]
**notes** | Option<**String**> |  | [optional]
**order_number** | Option<**String**> | Reference to the order/quote this proforma belongs to. | [optional]
**payment_due_date** | Option<**chrono::NaiveDate**> | Optional deadline the real invoice should carry after conversion. | [optional]
**quotation_id** | Option<**String**> | References the quotation entity. | [optional]
**status** | Option<[**models::ProformaInvoiceStatus**](ProformaInvoiceStatus.md)> | `draft` | `sent` | `converted`. | [optional]
**subtotal** | Option<**String**> |  | [optional]
**total_amount** | Option<**String**> |  | [optional]
**total_tax** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


