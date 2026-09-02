# \LegalDocumentApi

All URIs are relative to *https://demo.simplebilly.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_legal_documents**](LegalDocumentApi.md#get_legal_documents) | **GET** /api/v1/legal/documents | List all legal documents of the tenant. Missing documents are seeded from the default texts (with tenant placeholders replaced) on first access.
[**reset_legal_documents**](LegalDocumentApi.md#reset_legal_documents) | **POST** /api/v1/legal/documents/reset | Restore default texts for all documents (or a single doc_type/lang when the optional filter is given). Returns the full tenant list.
[**upsert_legal_documents**](LegalDocumentApi.md#upsert_legal_documents) | **PUT** /api/v1/legal/documents | Upsert legal documents per (doc_type, lang). Returns the full tenant list.



## get_legal_documents

> Vec<models::LegalDocument> get_legal_documents()
List all legal documents of the tenant. Missing documents are seeded from the default texts (with tenant placeholders replaced) on first access.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::LegalDocument>**](LegalDocument.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_legal_documents

> Vec<models::LegalDocument> reset_legal_documents(legal_document_reset)
Restore default texts for all documents (or a single doc_type/lang when the optional filter is given). Returns the full tenant list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**legal_document_reset** | [**LegalDocumentReset**](LegalDocumentReset.md) |  | [required] |

### Return type

[**Vec<models::LegalDocument>**](LegalDocument.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upsert_legal_documents

> Vec<models::LegalDocument> upsert_legal_documents(legal_document_upsert)
Upsert legal documents per (doc_type, lang). Returns the full tenant list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**legal_document_upsert** | [**Vec<models::LegalDocumentUpsert>**](LegalDocumentUpsert.md) |  | [required] |

### Return type

[**Vec<models::LegalDocument>**](LegalDocument.md)

### Authorization

[bearer_token](../README.md#bearer_token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

