# GdprExport

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**activity_log** | [**Vec<models::GdprActivity>**](GdprActivity.md) |  | 
**api_keys** | [**Vec<models::GdprApiKey>**](GdprApiKey.md) | Key identifiers and names only — never a usable credential. | 
**billing** | [**Vec<models::GdprBillingInfo>**](GdprBillingInfo.md) |  | 
**exported_at** | **chrono::DateTime<chrono::FixedOffset>** |  | 
**generated_by_ai** | **bool** | Honesty field: this document is a plain data dump, never AI-generated. | 
**notifications** | [**Vec<models::GdprNotification>**](GdprNotification.md) |  | 
**refresh_tokens** | [**Vec<models::GdprRefreshToken>**](GdprRefreshToken.md) | Session records: metadata only, never the token hash. | 
**tenants** | [**Vec<models::GdprTenant>**](GdprTenant.md) |  | 
**usage_events** | [**Vec<models::GdprUsageEvent>**](GdprUsageEvent.md) |  | 
**user** | [**models::GdprUser**](GdprUser.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


