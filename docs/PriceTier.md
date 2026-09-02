# PriceTier

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**customer_group_id** | Option<**String**> | None = tier applies to all customers; otherwise a customer group id. | [optional]
**min_quantity** | Option<**i64**> | Quantity from which this tier applies (inclusive). | [optional]
**product_id** | **uuid::Uuid** | References the product entity. | 
**unit_price** | **String** | Net unit price once `min_quantity` is reached. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


