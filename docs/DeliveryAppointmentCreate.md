# DeliveryAppointmentCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**email** | **String** |  | 
**notes** | Option<**String**> |  | [optional]
**phone** | Option<**String**> |  | [optional]
**requested_date** | **chrono::NaiveDate** |  | 
**status** | [**models::DeliveryAppointmentStatus**](DeliveryAppointmentStatus.md) | One of: requested | confirmed | arrived | cancelled | completed | 
**supplier_name** | **String** |  | 
**time_slot** | Option<**String**> | e.g. \"08:00-10:00\" | [optional]
**warehouse_id** | **String** | References the warehouse entity. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


