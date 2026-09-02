# DeliveryNote

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**address** | Option<**serde_json::Value**> |  | [optional]
**contact_id** | Option<**String**> | References the contact entity. | [optional]
**contact_name** | Option<**String**> |  | [optional]
**currency** | **String** |  | 
**delivery_date** | Option<**chrono::NaiveDate**> |  | [optional]
**delivery_note_number** | Option<**String**> |  | [optional]
**files** | Option<**serde_json::Value**> |  | [optional]
**introduction** | Option<**String**> |  | [optional]
**line_items** | Option<**serde_json::Value**> |  | [optional]
**preceding_sales_voucher_id** | Option<**String**> | References the preceding sales voucher entity. | [optional]
**preceding_sales_voucher_type** | Option<[**models::PrecedingSalesVoucherType**](PrecedingSalesVoucherType.md)> |  | [optional]
**remark** | Option<**String**> |  | [optional]
**shipping_date** | Option<**chrono::NaiveDate**> |  | [optional]
**shipping_method** | Option<**String**> |  | [optional]
**subtotal** | Option<**String**> |  | [optional]
**title** | Option<**String**> |  | [optional]
**total_amount** | Option<**String**> |  | [optional]
**voucher_date** | **chrono::NaiveDate** |  | 
**voucher_status** | [**models::VoucherStatus**](VoucherStatus.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


