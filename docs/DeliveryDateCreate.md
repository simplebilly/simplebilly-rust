# DeliveryDateCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**customer_id** | Option<**String**> | References the customer entity. | [optional]
**fulfilled_date** | Option<**chrono::NaiveDate**> | Date actually delivered (set on fulfillment). | [optional]
**note** | Option<**String**> |  | [optional]
**order_number** | **String** | Sales order number (`order.order_number`). | 
**original_date** | Option<**chrono::NaiveDate**> | Original date promised before rescheduling. | [optional]
**product_id** | Option<**String**> | Product line item this date applies to, if per-item. References the product entity. | [optional]
**promised_date** | **chrono::NaiveDate** | Date promised to the customer. | 
**status** | Option<[**models::DeliveryDateStatus**](DeliveryDateStatus.md)> | One of: promised | confirmed | rescheduled | fulfilled | late | cancelled | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


