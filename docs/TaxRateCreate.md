# TaxRateCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**country_code** | **String** | ISO 3166-1 alpha-2 country code. | 
**effective_from** | Option<**chrono::NaiveDate**> | Date this rate took effect; `None` = not date-bound. | [optional]
**is_default** | **bool** | Default rate for the country (one per country); fallback for lookups when no dated rate applies. | 
**name** | **String** | Human name, e.g. \"VAT\". | 
**rate_percent** | **i64** | Rate in hundredths of a percent: 1900 = 19.00%. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


