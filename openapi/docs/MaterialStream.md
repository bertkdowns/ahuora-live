# MaterialStream

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**x** | Option<**String**> |  | [optional][default to 0.00000]
**y** | Option<**String**> |  | [optional][default to 0.00000]
**graphic_object** | Option<[**models::GraphicObject**](GraphicObject.md)> |  | [optional]
**properties** | Option<[**models::PropertySet**](PropertySet.md)> |  | [optional]
**composition** | Option<[**Vec<models::InputComposition>**](InputComposition.md)> |  | [optional]
**name** | **String** |  | [readonly]
**component_name** | Option<**String**> |  | [optional]
**at_equilibrium** | Option<**bool**> |  | [optional]
**is_electrolyte_stream** | Option<**bool**> |  | [optional]
**reference_solvent** | Option<**String**> |  | [optional]
**floating_table_amount_basis** | Option<[**models::FloatingTableAmountBasisEnum**](FloatingTableAmountBasisEnum.md)> |  | [optional]
**defined_flow** | Option<[**models::DefinedFlowEnum**](DefinedFlowEnum.md)> |  | [optional]
**composition_basis** | Option<[**models::CompositionBasisEnum**](CompositionBasisEnum.md)> |  | [optional]
**force_phase** | Option<[**models::ForcePhaseEnum**](ForcePhaseEnum.md)> |  | [optional]
**calculation_mode** | Option<[**models::MaterialStreamCalculationModeEnum**](MaterialStreamCalculationModeEnum.md)> |  | [optional]
**object_class** | [**models::ObjectClassEnum**](ObjectClassEnum.md) |  | [readonly]
**substream_flag** | Option<[**models::SubstreamFlagEnum**](SubstreamFlagEnum.md)> |  | [optional]
**flowsheet_owner** | Option<**i32**> |  | [optional]
**from_port** | Option<**i32**> |  | [optional]
**to_port** | Option<**i32**> |  | [optional]
**parent_stream** | Option<**i32**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


