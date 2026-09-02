# ProductAttributeUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**is_filterable** | Option<**bool**> | Whether this attribute participates in the shop's faceted filters. | [optional]
**name** | Option<**String**> | Attribute name, e.g. `Material`, `Farbe`, `Gewicht`. | [optional]
**position** | Option<**i32**> | Ordering position within the product's attribute list. | [optional]
**product_id** | Option<**uuid::Uuid**> | The product this attribute belongs to. References the product entity. | [optional]
**unit** | Option<**String**> | Optional unit of measure for numeric attributes, e.g. `g`, `cm`. | [optional]
**value** | Option<**String**> | Attribute value, e.g. `Baumwolle`, `Rot`, `180g`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


