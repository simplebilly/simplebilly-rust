# Voucher

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**category_id** | Option<**String**> |  | [optional]
**contact_id** | Option<**String**> | References the contact entity. | [optional]
**contact_name** | Option<**String**> |  | [optional]
**currency** | **String** |  | 
**description** | Option<**String**> |  | [optional]
**file_attachments** | Option<**serde_json::Value**> |  | [optional]
**line_items** | Option<**serde_json::Value**> |  | [optional]
**metadata** | Option<**serde_json::Value**> |  | [optional]
**notes** | Option<**String**> |  | [optional]
**open_amount** | Option<**String**> |  | [optional]
**paid_date** | Option<**chrono::NaiveDate**> |  | [optional]
**payment_status** | Option<[**models::PaymentStatus**](PaymentStatus.md)> |  | [optional]
**tax_amounts** | Option<**serde_json::Value**> |  | [optional]
**tax_condition** | Option<**String**> |  | [optional]
**total_gross_amount** | Option<**String**> |  | [optional]
**total_net_amount** | Option<**String**> |  | [optional]
**voucher_date** | **chrono::NaiveDate** |  | 
**voucher_number** | Option<**String**> |  | [optional]
**voucher_status** | [**models::VoucherStatus**](VoucherStatus.md) |  | 
**voucher_type** | [**models::VoucherType**](VoucherType.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


