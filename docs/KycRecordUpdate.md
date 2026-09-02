# KycRecordUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**customer_id** | Option<**String**> | Referenz auf den Kunden/Kontakt. | [optional]
**customer_name** | Option<**String**> | Name des Kunden (für die Suche). | [optional]
**kyc_date** | Option<**chrono::NaiveDate**> | Datum der KYC-Prüfung (GwG § 8). | [optional]
**notes** | Option<**String**> | Freitext-Notizen. | [optional]
**retention_until** | Option<**chrono::NaiveDate**> | Aufbewahrungsfrist (GwG § 8 Abs. 4: 5 Jahre). | [optional]
**risk_assessment** | Option<**String**> | Risikoeinschätzung (z. B. Risikoklasse). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


