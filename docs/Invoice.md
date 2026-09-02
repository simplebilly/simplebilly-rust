# Invoice

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**attachments** | Option<**serde_json::Value**> |  | [optional]
**billing_period_end** | Option<**chrono::NaiveDate**> |  | [optional]
**billing_period_start** | Option<**chrono::NaiveDate**> |  | [optional]
**cancellation_date** | Option<**chrono::NaiveDate**> |  | [optional]
**cancellation_invoice_id** | Option<**String**> | References the invoice entity. | [optional]
**cancellation_reason** | Option<**String**> |  | [optional]
**contract_id** | Option<**uuid::Uuid**> | References the contract entity. | [optional]
**currency** | [**models::CurrencyCode**](CurrencyCode.md) |  | 
**customer_id** | Option<**String**> | References the customer entity. | [optional]
**discount_amount** | Option<**String**> |  | [optional]
**discount_days** | Option<**i32**> |  | [optional]
**discount_percentage** | Option<**String**> |  | [optional]
**document_type** | Option<[**models::DocumentType**](DocumentType.md)> |  | [optional]
**dunning_level** | Option<**i32**> |  | [optional]
**input_vat_amount** | Option<**String**> |  | [optional]
**input_vat_deductible** | Option<**bool**> |  | [optional]
**input_vat_percentage** | Option<**String**> |  | [optional]
**introduction_text** | Option<**String**> |  | [optional]
**invoice_type** | [**models::InvoiceType**](InvoiceType.md) |  | 
**is_cancelled** | Option<**bool**> |  | [optional]
**is_draft** | Option<**bool**> |  | [optional]
**is_eu_acquisition** | Option<**bool**> |  | [optional]
**is_eu_delivery** | Option<**bool**> |  | [optional]
**is_intra_community_acquisition** | Option<**bool**> |  | [optional]
**is_reverse_charge** | Option<**bool**> |  | [optional]
**issue_date** | **chrono::NaiveDate** |  | 
**ledger_account** | Option<**String**> |  | [optional]
**line_items** | Option<**serde_json::Value**> |  | 
**margin25a** | Option<**bool**> |  | [optional]
**margin25a_gross** | Option<**String**> |  | [optional]
**margin25a_purchase_price** | Option<**String**> |  | [optional]
**notes** | Option<**String**> |  | [optional]
**order_number** | Option<**String**> |  | [optional]
**original_pdf_path** | Option<**String**> |  | [optional]
**paid_amount** | Option<**String**> |  | [optional]
**payment_due_date** | Option<**chrono::NaiveDate**> |  | [optional]
**payment_status** | Option<[**models::PaymentStatus**](PaymentStatus.md)> |  | [optional]
**payment_terms_text** | Option<**String**> |  | [optional]
**preceding_sales_voucher_id** | Option<**String**> | References the preceding sales voucher entity. | [optional]
**preceding_sales_voucher_type** | Option<[**models::PrecedingSalesVoucherType**](PrecedingSalesVoucherType.md)> |  | [optional]
**receipt_confirmation_available** | Option<**bool**> |  | [optional]
**related_invoice_id** | Option<**uuid::Uuid**> | References the invoice entity. | [optional]
**relationship_type** | Option<**String**> |  | [optional]
**sender_snapshot** | Option<**serde_json::Value**> |  | [optional]
**sent_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**service_period_end** | Option<**chrono::NaiveDate**> |  | [optional]
**service_period_start** | Option<**chrono::NaiveDate**> |  | [optional]
**status** | [**models::InvoiceStatus**](InvoiceStatus.md) |  | 
**subtotal** | **String** |  | 
**supplier_id** | Option<**String**> | References the supplier entity. | [optional]
**tax_exemption_reason** | Option<**String**> |  | [optional]
**total_amount** | **String** |  | 
**total_tax** | **String** |  | 
**vat_country** | Option<[**models::CountryCode**](CountryCode.md)> |  | [optional]
**vat_special_case** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


