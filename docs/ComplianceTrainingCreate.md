# ComplianceTrainingCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**assignable** | Option<**bool**> | Whether HR can assign this training as required for employees. | [optional]
**code** | Option<**String**> | Stable code used by plugins and frontend players (e.g. \"data_privacy\"). | [optional]
**description** | Option<**String**> |  | [optional]
**pass_score** | Option<**i32**> | Minimum score (0–100) required to pass. | [optional]
**plugin_platform** | Option<**String**> | Marketplace plugin platform id when source = Plugin. | [optional]
**source** | Option<[**models::TrainingSource**](TrainingSource.md)> |  | [optional]
**title** | Option<**String**> |  | [optional]
**validity_months** | Option<**i32**> | Certificate validity in months; null = no expiry. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


