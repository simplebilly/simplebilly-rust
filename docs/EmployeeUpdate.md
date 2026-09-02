# EmployeeUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**address** | Option<**String**> |  | [optional]
**backup_employee_id** | Option<**uuid::Uuid**> | References another employee who covers when this employee is absent. | [optional]
**bic** | Option<**String**> |  | [optional]
**city** | Option<**String**> |  | [optional]
**country** | Option<[**models::CountryCode**](CountryCode.md)> |  | [optional]
**date_of_birth** | Option<**chrono::NaiveDate**> |  | [optional]
**department_id** | Option<**uuid::Uuid**> | References the department entity. | [optional]
**email** | Option<**String**> |  | [optional]
**first_name** | Option<**String**> |  | [optional]
**gender** | Option<[**models::Gender**](Gender.md)> | Gender for pay-transparency reporting: \"male\", \"female\" or \"diverse\". | [optional]
**hire_date** | Option<**chrono::NaiveDate**> |  | [optional]
**hourly_cost** | Option<**String**> | Hourly cost rate in EUR for labor-cost reporting; when unset the rate is derived from `monthly_salary / (weekly_hours * 4.33)`. | [optional]
**iban** | Option<**String**> |  | [optional]
**job_title** | Option<**String**> |  | [optional]
**last_login** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**last_name** | Option<**String**> |  | [optional]
**last_updated** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**monthly_salary** | Option<**String**> | Gross monthly salary in EUR for pay-transparency reporting. | [optional]
**phone** | Option<**String**> |  | [optional]
**state** | Option<**String**> |  | [optional]
**status** | Option<[**models::EmployeeStatus**](EmployeeStatus.md)> |  | [optional]
**user_id** | Option<**uuid::Uuid**> | References the user entity. | [optional]
**weekly_hours** | Option<**String**> | Contractual weekly working hours for pay-transparency normalization. | [optional]
**zip** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


