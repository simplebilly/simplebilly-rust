# ServiceJobUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**address** | Option<**String**> | Street + zip + city of the job location. | [optional]
**customer_email** | Option<**String**> | Customer email for email notifications. | [optional]
**customer_id** | Option<**uuid::Uuid**> | References the customer entity. | [optional]
**customer_name** | Option<**String**> | Denormalized customer name for quick display. | [optional]
**customer_phone** | Option<**String**> | Customer phone for SMS notifications later. | [optional]
**description** | Option<**String**> | What work needs to be done. | [optional]
**estimated_duration_minutes** | Option<**i32**> | Estimated time for the job in minutes. | [optional]
**lat** | Option<**f64**> | Latitude for map display (OpenStreetMap). | [optional]
**lng** | Option<**f64**> | Longitude for map display (OpenStreetMap). | [optional]
**notes** | Option<**String**> |  | [optional]
**status** | Option<[**models::ServiceJobStatus**](ServiceJobStatus.md)> | Dispatch status: \"pending\", \"assigned\", \"en_route\", \"in_progress\", \"completed\", \"cancelled\". | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


