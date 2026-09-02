# \ShipmentApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_shipment**](ShipmentApi.md#create_shipment) | **POST** /api/v1/shipments | 
[**create_shipment_from_order**](ShipmentApi.md#create_shipment_from_order) | **POST** /api/v1/orders/{order_number}/shipments | Create a real shipment for an order: calls the configured carrier's label API, stores the returned tracking/label on a new shipment row, and marks the order as shipped.
[**delete_shipment**](ShipmentApi.md#delete_shipment) | **DELETE** /api/v1/shipments/{shipment_id} | 
[**get_shipment**](ShipmentApi.md#get_shipment) | **GET** /api/v1/shipments/{shipment_id} | 
[**list_shipments**](ShipmentApi.md#list_shipments) | **GET** /api/v1/shipments | 
[**track_order_public**](ShipmentApi.md#track_order_public) | **POST** /api/v1/public/track | Customer-facing tracking lookup: order number + email → shipment status and live carrier events. No auth (public storefront API).
[**track_shipment_api**](ShipmentApi.md#track_shipment_api) | **GET** /api/v1/shipments/{shipment_id}/tracking | 
[**update_shipment_status**](ShipmentApi.md#update_shipment_status) | **PUT** /api/v1/shipments/{shipment_id}/status | 



## create_shipment

> models::Shipment create_shipment(shipment)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**shipment** | [**Shipment**](Shipment.md) |  | [required] |

### Return type

[**models::Shipment**](Shipment.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_shipment_from_order

> models::Shipment create_shipment_from_order(order_number, create_shipment_request)
Create a real shipment for an order: calls the configured carrier's label API, stores the returned tracking/label on a new shipment row, and marks the order as shipped.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_number** | **String** |  | [required] |
**create_shipment_request** | [**CreateShipmentRequest**](CreateShipmentRequest.md) |  | [required] |

### Return type

[**models::Shipment**](Shipment.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_shipment

> delete_shipment(shipment_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**shipment_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_shipment

> models::Shipment get_shipment(shipment_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**shipment_id** | **String** |  | [required] |

### Return type

[**models::Shipment**](Shipment.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_shipments

> Vec<models::Shipment> list_shipments(page, page_size, search, include_deleted)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |
**search** | Option<**String**> |  |  |
**include_deleted** | Option<**bool**> | Soft-delete entities: set true to include rows with `deleted_at` set. |  |

### Return type

[**Vec<models::Shipment>**](Shipment.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## track_order_public

> models::TrackOrderResponse track_order_public(track_order_request)
Customer-facing tracking lookup: order number + email → shipment status and live carrier events. No auth (public storefront API).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**track_order_request** | [**TrackOrderRequest**](TrackOrderRequest.md) |  | [required] |

### Return type

[**models::TrackOrderResponse**](TrackOrderResponse.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## track_shipment_api

> models::TrackingInfo track_shipment_api(shipment_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**shipment_id** | **String** |  | [required] |

### Return type

[**models::TrackingInfo**](TrackingInfo.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_shipment_status

> models::Shipment update_shipment_status(shipment_id, shipment_status_update)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**shipment_id** | **String** |  | [required] |
**shipment_status_update** | [**ShipmentStatusUpdate**](ShipmentStatusUpdate.md) |  | [required] |

### Return type

[**models::Shipment**](Shipment.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

