# ProductionOrder

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bom_id** | Option<**uuid::Uuid**> | References the BOM entity. | [optional]
**components** | Option<**serde_json::Value**> | JSON snapshot of the BOM components at creation time. | [optional]
**end_date** | Option<**chrono::NaiveDate**> |  | [optional]
**notes** | Option<**String**> |  | [optional]
**order_number** | **String** |  | 
**product_id** | **uuid::Uuid** | The finished product to manufacture. References the product entity. | 
**quantity** | **i64** | Quantity of finished product to produce. | 
**source_warehouse_id** | Option<**String**> | Warehouse components are consumed from. References the warehouse entity. | [optional]
**start_date** | Option<**chrono::NaiveDate**> |  | [optional]
**status** | Option<[**models::ProductionOrderStatus**](ProductionOrderStatus.md)> | One of: planned | in_production | completed | cancelled | [optional]
**target_warehouse_id** | Option<**String**> | Warehouse the finished product is added to. References the warehouse entity. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


