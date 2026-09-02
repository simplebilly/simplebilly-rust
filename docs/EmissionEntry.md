# EmissionEntry

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**activity_value** | **String** | Activity amount in `unit` (kWh, l, km, t, tkm, EUR). | 
**category_id** | **String** | GHG-Protocol category key, e.g. \"purchased_goods\", \"business_travel\". | 
**description** | **String** |  | 
**ef_source** | **String** | Emission-factor source, e.g. \"UBA-2024\", \"DEFRA-2024\". | 
**ef_version** | **String** |  | 
**method** | [**models::EmissionMethod**](EmissionMethod.md) | \"activity\" | \"spend\" | \"supplier\". | 
**scope** | [**models::GhgScope**](GhgScope.md) | GHG scope: \"1\" | \"2\" | \"3\". | 
**tco2e** | **String** | Computed server-side: activity * factor / 1000, rounded to 4 dp. | 
**unit** | **String** | Unit of the activity value. | 
**updated_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**year** | **i32** | Reporting year. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


