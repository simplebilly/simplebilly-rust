# ApiResponseSubscriptionOverviewData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**current_period_end** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**features** | [**models::PlanFeatures**](PlanFeatures.md) |  | 
**is_trialing** | **bool** |  | 
**limits** | [**models::PlanLimits**](PlanLimits.md) |  | 
**manage_url** | Option<**String**> |  | [optional]
**plan** | **String** | Resolved plan id (free/starter/business/enterprise, or a custom override id). | 
**plan_name** | **String** |  | 
**price_eur** | **f64** | Monthly price in EUR; `-1.0` = custom pricing (enterprise). | 
**quantity** | Option<**i32**> |  | [optional]
**status** | Option<**String**> |  | [optional]
**subscription_id** | Option<**String**> |  | [optional]
**trial_ends_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**usage** | [**models::UsageSnapshot**](UsageSnapshot.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


