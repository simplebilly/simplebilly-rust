# QuotaOverride

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**features** | Option<[**models::QuotaOverrideFeatures**](QuotaOverrideFeatures.md)> |  | [optional]
**max_connectors** | Option<**i32**> |  | [optional]
**max_invoices_per_month** | Option<**i64**> |  | [optional]
**max_users** | Option<**i32**> |  | [optional]
**metered** | Option<**std::collections::HashMap<String, i64>**> |  | [optional]
**plan** | Option<**String**> | Custom plan id; unknown ids resolve to enterprise limits. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


