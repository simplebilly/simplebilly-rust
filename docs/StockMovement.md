# StockMovement

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**delta** | **i64** | Signed movement: positive = into stock, negative = out of stock. | 
**movement_type** | [**models::MovementType**](MovementType.md) | One of the `MOVEMENT_*` constants. | 
**product_id** | **uuid::Uuid** | References the product entity. | 
**quantity** | **i64** | Absolute quantity moved (always >= 0). | 
**reason** | Option<**String**> |  | [optional]
**reference_id** | Option<**String**> | Primary-key of the referencing entity. | [optional]
**reference_type** | Option<[**models::ReferenceType**](ReferenceType.md)> | Entity that caused the movement, e.g. `goods_receipt`, `stock_transfer`. | [optional]
**warehouse_id** | **String** | References the warehouse entity. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


