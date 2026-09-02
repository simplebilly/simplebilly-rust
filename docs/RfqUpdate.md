# RfqUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**currency** | Option<**String**> |  | [optional]
**line_items** | Option<**serde_json::Value**> | JSON array of `{product_id, name, sku, quantity, requested_unit_price?, quoted_unit_price?}`. | [optional]
**notes** | Option<**String**> |  | [optional]
**requested_date** | Option<**chrono::NaiveDate**> |  | [optional]
**response_date** | Option<**chrono::NaiveDate**> |  | [optional]
**rfq_number** | Option<**String**> |  | [optional]
**status** | Option<[**models::RfqStatus**](RfqStatus.md)> | One of: draft | sent | offer_received | rejected | converted | [optional]
**supplier_contact_id** | Option<**String**> | References the supplier entity. | [optional]
**supplier_name** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


