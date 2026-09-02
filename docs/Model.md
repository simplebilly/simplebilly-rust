# Model

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**backup_codes** | **Vec<String>** |  | 
**created_at** | **chrono::DateTime<chrono::FixedOffset>** |  | 
**deleted_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**email** | **String** |  | 
**email_verified** | **bool** |  | 
**id** | **uuid::Uuid** |  | 
**is_active** | **bool** |  | 
**is_totp_enabled** | **bool** |  | 
**last_login** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**name** | **String** |  | 
**oauth_id** | Option<**String**> |  | [optional]
**oauth_provider** | Option<**String**> |  | [optional]
**password_changed_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Set on password change; auth/refresh tokens issued before this timestamp are rejected by the auth middleware. | [optional]
**password_hash** | **String** |  | 
**picture** | Option<**String**> |  | [optional]
**privacy_accepted_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | When the user accepted the data privacy policy (GDPR consent record). | [optional]
**totp_secret** | Option<**String**> |  | [optional]
**updated_at** | **chrono::DateTime<chrono::FixedOffset>** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


