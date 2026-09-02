# MarketplaceConnection

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**config** | Option<**serde_json::Value**> |  | 
**connection_id** | **String** |  | 
**connector_type** | [**models::ConnectorType**](ConnectorType.md) |  | 
**created_at** | **chrono::DateTime<chrono::FixedOffset>** |  | 
**is_active** | **bool** |  | 
**label** | **String** |  | 
**last_sync_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**platform** | **String** |  | 
**platform_user_id** | Option<**String**> |  | [optional]
**scopes** | Option<**String**> |  | [optional]
**shop_domain** | Option<**String**> |  | [optional]
**shop_name** | Option<**String**> |  | [optional]
**sync_status** | Option<**String**> |  | [optional]
**tenant_id** | **uuid::Uuid** |  | 
**updated_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


