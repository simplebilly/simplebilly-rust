# Absence

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**absence_type** | Option<[**models::AbsenceType**](AbsenceType.md)> | One of \"vacation\", \"sick\", \"sabbatical\", \"parental\", \"other\". | [optional]
**approved_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**approved_by** | Option<**uuid::Uuid**> | References the user entity. | [optional]
**created_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**deleted_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**employee_id** | Option<**uuid::Uuid**> | References the employee entity. | [optional]
**end_date** | Option<**chrono::NaiveDate**> |  | [optional]
**id** | Option<**uuid::Uuid**> |  | [optional]
**notes** | Option<**String**> |  | [optional]
**start_date** | Option<**chrono::NaiveDate**> |  | [optional]
**status** | Option<[**models::AbsenceStatus**](AbsenceStatus.md)> | One of \"pending\", \"approved\", \"rejected\", \"cancelled\". | [optional]
**tenant_id** | Option<**uuid::Uuid**> |  | [optional]
**updated_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


