# PlanLimits

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**max_connectors** | **i32** |  | 
**max_invoices_per_month** | **i64** |  | 
**max_users** | **i32** |  | 
**metered** | Option<**std::collections::HashMap<String, i64>**> |  | [optional]
**paid_connectors** | **Vec<String>** | Connectors that are *not* included in this plan (require a higher tier). Empty = all connectors included on this plan. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


