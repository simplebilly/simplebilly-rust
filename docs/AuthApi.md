# \AuthApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**accept_invite**](AuthApi.md#accept_invite) | **POST** /auth/accept-invite | Accept an invite: create the account (or reuse an existing one) and join the inviting tenant. The invite token proves control of the mailbox.
[**forgot_password**](AuthApi.md#forgot_password) | **POST** /auth/forgot-password | Send a password reset email to the user
[**login**](AuthApi.md#login) | **POST** /auth/login | Authenticate a user with email + password (optional TOTP)
[**logout**](AuthApi.md#logout) | **POST** /auth/logout | Log out the current user (kills the assay session)
[**magic_link_login**](AuthApi.md#magic_link_login) | **POST** /auth/magic-link | Request a magic link login (sends an email with a one-time link)
[**magic_link_verify**](AuthApi.md#magic_link_verify) | **POST** /auth/magic-link/verify | Verify a magic link token and log the user in
[**register**](AuthApi.md#register) | **POST** /auth/register | Register a new user account
[**reset_password**](AuthApi.md#reset_password) | **POST** /auth/reset-password | Reset the user's password using a reset token
[**totp_enable**](AuthApi.md#totp_enable) | **POST** /auth/totp/enable | Enable TOTP two-factor authentication by verifying a code
[**totp_setup**](AuthApi.md#totp_setup) | **GET** /auth/totp/setup | Set up TOTP two-factor authentication (generates secret + backup codes)
[**verify_email**](AuthApi.md#verify_email) | **POST** /auth/verify-email | Verify a user's email address using a verification token



## accept_invite

> accept_invite(accept_invite_request)
Accept an invite: create the account (or reuse an existing one) and join the inviting tenant. The invite token proves control of the mailbox.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**accept_invite_request** | [**AcceptInviteRequest**](AcceptInviteRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## forgot_password

> forgot_password(forgot_password_request)
Send a password reset email to the user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**forgot_password_request** | [**ForgotPasswordRequest**](ForgotPasswordRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## login

> models::AuthResponse login(login_request)
Authenticate a user with email + password (optional TOTP)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**login_request** | [**LoginRequest**](LoginRequest.md) |  | [required] |

### Return type

[**models::AuthResponse**](AuthResponse.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## logout

> logout()
Log out the current user (kills the assay session)

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## magic_link_login

> magic_link_login(magic_link_request)
Request a magic link login (sends an email with a one-time link)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**magic_link_request** | [**MagicLinkRequest**](MagicLinkRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## magic_link_verify

> models::AuthResponse magic_link_verify(magic_link_verify_request)
Verify a magic link token and log the user in

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**magic_link_verify_request** | [**MagicLinkVerifyRequest**](MagicLinkVerifyRequest.md) |  | [required] |

### Return type

[**models::AuthResponse**](AuthResponse.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## register

> models::AuthResponse register(register_request)
Register a new user account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**register_request** | [**RegisterRequest**](RegisterRequest.md) |  | [required] |

### Return type

[**models::AuthResponse**](AuthResponse.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_password

> reset_password(reset_password_request)
Reset the user's password using a reset token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reset_password_request** | [**ResetPasswordRequest**](ResetPasswordRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## totp_enable

> totp_enable(totp_enable_request)
Enable TOTP two-factor authentication by verifying a code

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**totp_enable_request** | [**TotpEnableRequest**](TotpEnableRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## totp_setup

> models::TotpSetupResponse totp_setup()
Set up TOTP two-factor authentication (generates secret + backup codes)

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::TotpSetupResponse**](TotpSetupResponse.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verify_email

> verify_email(verify_email_request)
Verify a user's email address using a verification token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verify_email_request** | [**VerifyEmailRequest**](VerifyEmailRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

