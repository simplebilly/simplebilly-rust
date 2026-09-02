# KonzernStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**groessenbefreit** | **bool** |  | 
**kapitalmarktorientiert** | **bool** |  | 
**konzernabschlusspflicht** | **bool** |  | 
**missing_group_figures** | **bool** | Keine group_figures-Zeile für das Jahr vorhanden → keine Größenbefreiung. | 
**mutterunternehmen** | **bool** | Mutterunternehmen: mindestens eine beherrschte Beteiligung (§ 290 Abs. 1 HGB). | 
**parent_name** | Option<**String**> | Mutterunternehmen für die Zwischenholding-Befreiung (§ 291 HGB). | [optional]
**parent_situs** | Option<**String**> |  | [optional]
**participations** | [**Vec<models::KonzernBeteiligung>**](KonzernBeteiligung.md) |  | 
**thresholds** | [**models::KonzernThresholds**](KonzernThresholds.md) |  | 
**year** | **i32** |  | 
**zwischenholding_befreit** | **bool** |  | 
**zwischenholding_hinweis** | Option<**String**> | Hinweis zu den § 291-Voraussetzungen (EU/EWR-Sitz, geprüfter Konzernabschluss). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


