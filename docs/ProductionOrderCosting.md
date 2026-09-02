# ProductionOrderCosting

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cost_per_unit** | **String** | material_cost_total ÷ quantity. | 
**cost_source** | **String** | \"actual\" when costed from stock-movement consumption, else \"planned\". | 
**lines** | [**Vec<models::CostingLine>**](CostingLine.md) |  | 
**margin_per_unit** | Option<**String**> | sale_price − cost_per_unit. | [optional]
**margin_percent** | Option<**String**> | margin_per_unit ÷ cost_per_unit as a percentage. | [optional]
**material_cost_total** | **String** | Total material cost for the whole order. | 
**order_number** | **String** |  | 
**production_order_id** | **uuid::Uuid** |  | 
**quantity** | **i64** |  | 
**sale_price** | Option<**String**> | Finished product's sale price per unit (used to compute margin). | [optional]
**status** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


