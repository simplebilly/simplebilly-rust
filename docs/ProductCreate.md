# ProductCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**availability** | Option<**String**> |  | [optional]
**barcode** | Option<**String**> |  | [optional]
**brand** | Option<**String**> |  | [optional]
**category_id** | Option<**String**> |  | [optional]
**condition** | Option<**String**> |  | [optional]
**default_ledger_account** | Option<**String**> |  | [optional]
**default_price** | Option<**String**> |  | [optional]
**default_price_formula_id** | Option<**uuid::Uuid**> | References the price formula entity. | [optional]
**default_tax_rate** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**gtin** | Option<**String**> |  | [optional]
**height** | Option<**String**> |  | [optional]
**image_link** | Option<**String**> |  | [optional]
**images** | Option<**serde_json::Value**> |  | [optional]
**is_taxable** | Option<**bool**> |  | [optional]
**length** | Option<**String**> |  | [optional]
**link** | Option<**String**> |  | [optional]
**max_stock** | Option<**i64**> | Target stock level used by reorder proposals. | [optional]
**min_stock** | Option<**i64**> | Reorder point — when stock falls below this, a reorder is suggested. | [optional]
**mpn** | Option<**String**> |  | [optional]
**name** | **String** |  | 
**package_height** | Option<**String**> |  | [optional]
**package_length** | Option<**String**> |  | [optional]
**package_weight_unit** | Option<**String**> |  | [optional]
**package_weight_value** | Option<**String**> |  | [optional]
**package_width** | Option<**String**> |  | [optional]
**product_code** | **String** |  | 
**product_type** | Option<**String**> |  | [optional]
**purchase_price** | Option<**String**> |  | [optional]
**reorder_quantity** | Option<**i64**> | Suggested purchase quantity when a reorder proposal is created. | [optional]
**sale_price** | Option<**String**> |  | [optional]
**shipping_price** | Option<**String**> |  | [optional]
**shipping_requires_insurance** | Option<**bool**> |  | [optional]
**sku** | **String** |  | 
**stock_quantity** | Option<**i64**> |  | [optional]
**tags** | Option<**serde_json::Value**> |  | [optional]
**tax_price** | Option<**String**> |  | [optional]
**track_batch** | Option<**bool**> | Whether this product requires batch (Chargennummer) tracking. | [optional]
**track_serial** | Option<**bool**> | Whether this product requires serial-number tracking. | [optional]
**unit** | Option<**serde_json::Value**> |  | [optional]
**weight_unit** | Option<**String**> |  | [optional]
**weight_value** | Option<**String**> |  | [optional]
**width** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


