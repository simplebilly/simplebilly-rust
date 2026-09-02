# SilentPartnerUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**contract_date** | Option<**chrono::NaiveDate**> | Datum des Vertragsabschlusses. | [optional]
**einlage** | Option<**String**> | Einlage (§ 230 HGB). | [optional]
**gewinnquote_pct** | Option<**String**> | Gewinnbeteiligungsquote in Prozent (§ 231 HGB). | [optional]
**gewinnvortrag** | Option<**String**> | Nicht erhobene Gewinne (§ 232 Abs. 3 HGB). | [optional]
**instrument_type** | Option<[**models::InstrumentType**](InstrumentType.md)> | Instrument: \"typisch\" | \"atypisch\" | \"partiarisches_darlehen\" | \"genussrecht\". | [optional]
**kest_pflichtig** | Option<**bool**> | 25 % Kapitalertragsteuer einbehalten (§ 43 Abs. 1 Nr. 3 EStG; typisch + partiarisches Darlehen). | [optional]
**name** | Option<**String**> | Name des stillen Gesellschafters. | [optional]
**notes** | Option<**String**> | Freitext-Notizen. | [optional]
**verlust_verrechnungskonto** | Option<**String**> | Kumulierte Verluste gegen die Einlage (§ 232 Abs. 2 HGB, ≤ Einlage). | [optional]
**verlustbeteiligung** | Option<**bool**> | Verlustbeteiligung (§ 231 Abs. 2 HGB; kann ausgeschlossen werden). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


