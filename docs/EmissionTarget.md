# EmissionTarget

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**base_value** | **String** |  | 
**base_year** | **i32** | tCO2e in the base year (actuals). | 
**description** | **String** | Transition-plan narrative (ESRS E1-1 light), may be empty. | 
**scope** | [**models::EmissionTargetScope**](EmissionTargetScope.md) | \"total\" | \"1\" | \"2\" | \"3\". | 
**target_value** | **String** |  | 
**target_year** | **i32** | tCO2e target for the target year. | 
**updated_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


