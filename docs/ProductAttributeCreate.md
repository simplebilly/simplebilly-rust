# ProductAttributeCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**is_filterable** | Option<**bool**> | Whether this attribute participates in the shop's faceted filters. | [optional]
**name** | **String** | Attribute name, e.g. `Material`, `Farbe`, `Gewicht`. | 
**position** | Option<**i32**> | Ordering position within the product's attribute list. | [optional]
**product_id** | **uuid::Uuid** | The product this attribute belongs to. References the product entity. | 
**unit** | Option<**String**> | Optional unit of measure for numeric attributes, e.g. `g`, `cm`. | [optional]
**value** | **String** | Attribute value, e.g. `Baumwolle`, `Rot`, `180g`. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


