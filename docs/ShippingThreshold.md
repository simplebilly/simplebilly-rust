# ShippingThreshold

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**is_active** | Option<**bool**> |  | [optional]
**max_sellable** | Option<**i64**> | Optional ceiling for the deliverable quantity. | [optional]
**name** | **String** |  | 
**notes** | Option<**String**> |  | [optional]
**product_id** | Option<**uuid::Uuid**> | None = applies to all products. References the product entity. | [optional]
**reserve_stock** | Option<**i64**> | Buffer of stock that must not be sold. | [optional]
**warehouse_id** | Option<**String**> | None = applies to all warehouses. References the warehouse entity. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


