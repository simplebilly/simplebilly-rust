# Shipment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**delivered_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**label_url** | Option<**String**> |  | [optional]
**line_items_shipment** | Option<**serde_json::Value**> |  | [optional]
**order_id** | **String** | References the order entity. | 
**recipient_address** | Option<**serde_json::Value**> |  | [optional]
**shipment_date** | **chrono::NaiveDate** |  | 
**shipping_carrier** | **String** |  | 
**shipping_cost** | Option<**String**> |  | [optional]
**shipping_method** | Option<**String**> |  | [optional]
**signed_by** | Option<**String**> |  | [optional]
**status** | **String** |  | 
**tracking_events** | Option<**serde_json::Value**> | Latest carrier tracking events (from the live tracking API). | [optional]
**tracking_number** | Option<**String**> |  | [optional]
**tracking_url** | Option<**String**> |  | [optional]
**weight_kg** | Option<**f64**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


