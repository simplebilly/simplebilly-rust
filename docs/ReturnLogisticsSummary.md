# ReturnLogisticsSummary

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**by_status** | Option<**serde_json::Value**> | Number of return orders per status. | 
**by_warehouse** | [**Vec<models::ReturnWarehouseSummary>**](ReturnWarehouseSummary.md) | Per-warehouse aggregation. | 
**items_restocked** | **i64** | Sum of `restock: true` line-item quantities. | 
**items_scrapped** | **i64** | Sum of `restock: false` line-item quantities (scrapped/disposed). | 
**total_items** | **i64** | Sum of all line-item quantities across returns. | 
**total_returns** | **i64** | Total number of return orders (excluding soft-deleted). | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


