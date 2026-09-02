# ReplenishmentSuggestionLine

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**current_stock** | **i64** | Current stock in the target warehouse. | 
**max_stock** | Option<**i64**> |  | [optional]
**min_stock** | Option<**i64**> |  | [optional]
**product_id** | **uuid::Uuid** |  | 
**product_name** | **String** |  | 
**sku** | **String** |  | 
**source_available** | **i64** | Surplus available in the source warehouse (above its target). | 
**source_warehouse_id** | **String** |  | 
**suggested_quantity** | **i64** |  | 
**target_warehouse_id** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


