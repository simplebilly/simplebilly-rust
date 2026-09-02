# OrderConfirmationCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**address** | Option<**serde_json::Value**> |  | [optional]
**confirmation_number** | Option<**String**> |  | [optional]
**contact_id** | Option<**String**> | References the contact entity. | [optional]
**contact_name** | Option<**String**> |  | [optional]
**currency** | **String** |  | 
**files** | Option<**serde_json::Value**> |  | [optional]
**introduction** | Option<**String**> |  | [optional]
**line_items** | Option<**serde_json::Value**> |  | [optional]
**preceding_sales_voucher_id** | Option<**String**> | References the preceding sales voucher entity. | [optional]
**preceding_sales_voucher_type** | Option<[**models::PrecedingSalesVoucherType**](PrecedingSalesVoucherType.md)> |  | [optional]
**remark** | Option<**String**> |  | [optional]
**tax_condition** | Option<**String**> |  | [optional]
**title** | Option<**String**> |  | [optional]
**voucher_date** | **chrono::NaiveDate** |  | 
**voucher_status** | [**models::VoucherStatus**](VoucherStatus.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


