# Declaration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**declaration_type** | Option<[**models::DeclarationType**](DeclarationType.md)> | Art der Erklärung: \"dcgk\" (Entsprechenserklärung § 161 AktG) oder \"unternehmensfuehrung\" (Erklärung zur Unternehmensführung § 289f HGB). | [optional]
**is_current** | Option<**bool**> | Kennzeichnet die aktuell gültige Fassung (max. eine je Mandant). | [optional]
**text** | Option<**String**> | Inhalt der Erklärung als Markdown. | [optional]
**valid_from** | Option<**chrono::NaiveDate**> | Datum, ab dem die Erklärung gilt. | [optional]
**version** | Option<**String**> | Versionsbezeichnung der Erklärung (z.B. \"2025-01\"). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


