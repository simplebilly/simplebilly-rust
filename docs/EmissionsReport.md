# EmissionsReport

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**by_category** | [**Vec<models::CategoryTotal>**](CategoryTotal.md) |  | 
**by_scope** | [**Vec<models::ScopeTotal>**](ScopeTotal.md) |  | 
**by_year** | [**Vec<models::YearTotal>**](YearTotal.md) |  | 
**data_quality** | [**models::DataQuality**](DataQuality.md) |  | 
**intensity_per_employee** | Option<**f64**> |  | [optional]
**intensity_per_revenue_mio** | Option<**f64**> | tCO2e per million EUR net revenue. | [optional]
**net_revenue** | Option<**f64**> | Sum of paid/sent/partially-paid invoices (EUR net) in the year. | [optional]
**spend_based_estimate_tco2e** | Option<**f64**> | Spend-based estimate from bookkeeping payments (EXIOBASE factor). | [optional]
**targets** | [**Vec<models::TargetProgress>**](TargetProgress.md) |  | 
**total_tco2e** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


