/*
 * Your Project API
 *
 * Your project description
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};


#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MaterialStream {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "x", skip_serializing_if = "Option::is_none")]
    pub x: Option<String>,
    #[serde(rename = "y", skip_serializing_if = "Option::is_none")]
    pub y: Option<String>,
    #[serde(rename = "graphicObject", skip_serializing_if = "Option::is_none")]
    pub graphic_object: Option<Box<models::GraphicObject>>,
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<Box<models::PropertySet>>,
    #[serde(rename = "composition", skip_serializing_if = "Option::is_none")]
    pub composition: Option<Vec<models::InputComposition>>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "componentName", skip_serializing_if = "Option::is_none")]
    pub component_name: Option<String>,
    #[serde(rename = "atEquilibrium", skip_serializing_if = "Option::is_none")]
    pub at_equilibrium: Option<bool>,
    #[serde(rename = "isElectrolyteStream", skip_serializing_if = "Option::is_none")]
    pub is_electrolyte_stream: Option<bool>,
    #[serde(rename = "referenceSolvent", skip_serializing_if = "Option::is_none")]
    pub reference_solvent: Option<String>,
    #[serde(rename = "floatingTableAmountBasis", skip_serializing_if = "Option::is_none")]
    pub floating_table_amount_basis: Option<models::FloatingTableAmountBasisEnum>,
    #[serde(rename = "definedFlow", skip_serializing_if = "Option::is_none")]
    pub defined_flow: Option<models::DefinedFlowEnum>,
    #[serde(rename = "compositionBasis", skip_serializing_if = "Option::is_none")]
    pub composition_basis: Option<models::CompositionBasisEnum>,
    #[serde(rename = "forcePhase", skip_serializing_if = "Option::is_none")]
    pub force_phase: Option<models::ForcePhaseEnum>,
    #[serde(rename = "calculationMode", skip_serializing_if = "Option::is_none")]
    pub calculation_mode: Option<models::MaterialStreamCalculationModeEnum>,
    #[serde(rename = "objectClass")]
    pub object_class: models::ObjectClassEnum,
    #[serde(rename = "substreamFlag", skip_serializing_if = "Option::is_none")]
    pub substream_flag: Option<models::SubstreamFlagEnum>,
    #[serde(rename = "flowsheetOwner", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub flowsheet_owner: Option<Option<i32>>,
    #[serde(rename = "fromPort", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub from_port: Option<Option<i32>>,
    #[serde(rename = "toPort", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub to_port: Option<Option<i32>>,
    #[serde(rename = "parentStream", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub parent_stream: Option<Option<i32>>,
}

impl MaterialStream {
    pub fn new(id: i32, name: String, object_class: models::ObjectClassEnum) -> MaterialStream {
        MaterialStream {
            id,
            x: None,
            y: None,
            graphic_object: None,
            properties: None,
            composition: None,
            name,
            component_name: None,
            at_equilibrium: None,
            is_electrolyte_stream: None,
            reference_solvent: None,
            floating_table_amount_basis: None,
            defined_flow: None,
            composition_basis: None,
            force_phase: None,
            calculation_mode: None,
            object_class,
            substream_flag: None,
            flowsheet_owner: None,
            from_port: None,
            to_port: None,
            parent_stream: None,
        }
    }
}

