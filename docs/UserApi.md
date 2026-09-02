# \UserApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**change_password**](UserApi.md#change_password) | **POST** /user/change-password | Change the current user's password (requires the current password).
[**create_team**](UserApi.md#create_team) | **POST** /user/teams | Create a new team within the current tenant
[**generate_api_key**](UserApi.md#generate_api_key) | **POST** /user/api-key | Generate a new API key for the current user
[**invite_user**](UserApi.md#invite_user) | **POST** /user/invite | Invite a user to the current tenant/organization
[**list_teams**](UserApi.md#list_teams) | **GET** /user/teams | List all teams in the current tenant
[**remove_user_from_org**](UserApi.md#remove_user_from_org) | **DELETE** /user/remove | Remove a user from the current organization
[**update_profile**](UserApi.md#update_profile) | **PUT** /user/profile | Update the current user's profile
[**user_profile**](UserApi.md#user_profile) | **GET** /user/profile | Get the current user's profile
[**user_tenants**](UserApi.md#user_tenants) | **GET** /user/tenants | List all tenants (organizations) the current user belongs to



## change_password

> change_password(change_password_request)
Change the current user's password (requires the current password).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**change_password_request** | [**ChangePasswordRequest**](ChangePasswordRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_team

> models::ApiResponseTeam create_team(team_create)
Create a new team within the current tenant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_create** | [**TeamCreate**](TeamCreate.md) |  | [required] |

### Return type

[**models::ApiResponseTeam**](ApiResponse_Team.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## generate_api_key

> models::ApiResponseString generate_api_key()
Generate a new API key for the current user

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ApiResponseString**](ApiResponse_String.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## invite_user

> invite_user(invite_request)
Invite a user to the current tenant/organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invite_request** | [**InviteRequest**](InviteRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_teams

> models::ApiResponseVecTeam list_teams()
List all teams in the current tenant

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ApiResponseVecTeam**](ApiResponse_Vec_Team.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_user_from_org

> remove_user_from_org(remove_user_request)
Remove a user from the current organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**remove_user_request** | [**RemoveUserRequest**](RemoveUserRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_profile

> update_profile(update_profile_request)
Update the current user's profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_profile_request** | [**UpdateProfileRequest**](UpdateProfileRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_profile

> models::ApiResponseUserProfile user_profile()
Get the current user's profile

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ApiResponseUserProfile**](ApiResponse_UserProfile.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_tenants

> models::ApiResponseVecUserTenantInfo user_tenants()
List all tenants (organizations) the current user belongs to

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ApiResponseVecUserTenantInfo**](ApiResponse_Vec_UserTenantInfo.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

