# GenerateVariantsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**options** | Option<[**std::collections::HashMap<String, Vec<String>>**](Vec.md)> | Option name → list of values, e.g. `{\"Color\": [\"Red\", \"Blue\"], \"Size\": [\"S\", \"M\"]}`. The cartesian product of these lists is generated. | [optional]
**price_delta** | Option<**String**> | Optional per-variant price delta applied to every generated variant. | [optional]
**product_id** | **uuid::Uuid** |  | 
**sku_prefix** | Option<**String**> | Optional prefix for the generated SKUs (suffix is the option values joined by `-`). Falls back to the parent product's SKU. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


