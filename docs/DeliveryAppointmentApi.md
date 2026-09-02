# \DeliveryAppointmentApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_delivery_appointment**](DeliveryAppointmentApi.md#create_delivery_appointment) | **POST** /api/v1/delivery-appointments | 
[**delete_delivery_appointment**](DeliveryAppointmentApi.md#delete_delivery_appointment) | **DELETE** /api/v1/delivery-appointments/{appointment_id} | 
[**get_delivery_appointment**](DeliveryAppointmentApi.md#get_delivery_appointment) | **GET** /api/v1/delivery-appointments/{appointment_id} | 
[**get_public_delivery_appointment_status**](DeliveryAppointmentApi.md#get_public_delivery_appointment_status) | **GET** /api/v1/public/delivery-appointments/status | Supplier/carrier checks appointment status (public, no auth). The appointment is only revealed when email AND token match.
[**list_delivery_appointments**](DeliveryAppointmentApi.md#list_delivery_appointments) | **GET** /api/v1/delivery-appointments | 
[**request_public_delivery_appointment**](DeliveryAppointmentApi.md#request_public_delivery_appointment) | **POST** /api/v1/public/delivery-appointments/request | Supplier/carrier requests an inbound delivery slot (public, no auth). The tenant is derived from the warehouse found by `code` — never from the request.
[**update_delivery_appointment**](DeliveryAppointmentApi.md#update_delivery_appointment) | **PUT** /api/v1/delivery-appointments/{appointment_id} | 
[**update_delivery_appointment_status**](DeliveryAppointmentApi.md#update_delivery_appointment_status) | **PUT** /api/v1/delivery-appointments/{appointment_id}/status | 



## create_delivery_appointment

> models::DeliveryAppointment create_delivery_appointment(delivery_appointment_create)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delivery_appointment_create** | [**DeliveryAppointmentCreate**](DeliveryAppointmentCreate.md) |  | [required] |

### Return type

[**models::DeliveryAppointment**](DeliveryAppointment.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_delivery_appointment

> delete_delivery_appointment(appointment_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**appointment_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_delivery_appointment

> models::DeliveryAppointment get_delivery_appointment(appointment_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**appointment_id** | **String** |  | [required] |

### Return type

[**models::DeliveryAppointment**](DeliveryAppointment.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_public_delivery_appointment_status

> models::PublicDeliveryAppointmentStatusResponse get_public_delivery_appointment_status(appointment_id, email, token)
Supplier/carrier checks appointment status (public, no auth). The appointment is only revealed when email AND token match.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**appointment_id** | **String** |  | [required] |
**email** | **String** |  | [required] |
**token** | **String** |  | [required] |

### Return type

[**models::PublicDeliveryAppointmentStatusResponse**](PublicDeliveryAppointmentStatusResponse.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_delivery_appointments

> Vec<models::DeliveryAppointment> list_delivery_appointments(page, page_size, status, warehouse_id, from, to)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**status** | Option<**String**> |  |  |
**warehouse_id** | Option<**String**> |  |  |
**from** | Option<**chrono::NaiveDate**> |  |  |
**to** | Option<**chrono::NaiveDate**> |  |  |

### Return type

[**Vec<models::DeliveryAppointment>**](DeliveryAppointment.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## request_public_delivery_appointment

> models::PublicDeliveryAppointmentResponse request_public_delivery_appointment(public_delivery_appointment_request)
Supplier/carrier requests an inbound delivery slot (public, no auth). The tenant is derived from the warehouse found by `code` — never from the request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**public_delivery_appointment_request** | [**PublicDeliveryAppointmentRequest**](PublicDeliveryAppointmentRequest.md) |  | [required] |

### Return type

[**models::PublicDeliveryAppointmentResponse**](PublicDeliveryAppointmentResponse.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_delivery_appointment

> models::DeliveryAppointment update_delivery_appointment(appointment_id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**appointment_id** | **String** |  | [required] |
**body** | Option<**serde_json::Value**> |  | [required] |

### Return type

[**models::DeliveryAppointment**](DeliveryAppointment.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_delivery_appointment_status

> models::DeliveryAppointment update_delivery_appointment_status(appointment_id, appointment_status_update)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**appointment_id** | **String** |  | [required] |
**appointment_status_update** | [**AppointmentStatusUpdate**](AppointmentStatusUpdate.md) |  | [required] |

### Return type

[**models::DeliveryAppointment**](DeliveryAppointment.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

