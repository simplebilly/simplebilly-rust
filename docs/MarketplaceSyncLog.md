# MarketplaceSyncLog

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**completed_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**connection_id** | **String** | References the marketplace connection entity. | 
**error_message** | Option<**String**> |  | [optional]
**items_failed** | **i32** |  | 
**items_synced** | **i32** |  | 
**platform** | **String** |  | 
**started_at** | **chrono::DateTime<chrono::FixedOffset>** |  | 
**status** | [**models::SyncLogStatus**](SyncLogStatus.md) |  | 
**sync_type** | [**models::SyncType**](SyncType.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


