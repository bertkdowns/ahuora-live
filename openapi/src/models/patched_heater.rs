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
pub struct PatchedHeater {
    #[serde(rename = "simulationObjectId", skip_serializing_if = "Option::is_none")]
    pub simulation_object_id: Option<i32>,
    #[serde(rename = "x", skip_serializing_if = "Option::is_none")]
    pub x: Option<String>,
    #[serde(rename = "y", skip_serializing_if = "Option::is_none")]
    pub y: Option<String>,
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<Box<models::PropertySet>>,
    #[serde(rename = "graphicObject", skip_serializing_if = "Option::is_none")]
    pub graphic_object: Option<Box<models::GraphicObject>>,
    #[serde(rename = "connectorPort", skip_serializing_if = "Option::is_none")]
    pub connector_port: Option<Vec<models::ConnectorPort>>,
    #[serde(rename = "componentName", skip_serializing_if = "Option::is_none")]
    pub component_name: Option<String>,
    #[serde(rename = "objectClass", skip_serializing_if = "Option::is_none")]
    pub object_class: Option<models::ObjectClassEnum>,
    #[serde(rename = "dynamicsSpec", skip_serializing_if = "Option::is_none")]
    pub dynamics_spec: Option<models::DynamicsSpecEnum>,
    #[serde(rename = "specVarType", skip_serializing_if = "Option::is_none")]
    pub spec_var_type: Option<models::SpecVarTypeEnum>,
    #[serde(rename = "adjustVarType", skip_serializing_if = "Option::is_none")]
    pub adjust_var_type: Option<models::AdjustVarTypeEnum>,
    #[serde(rename = "hasPropertiesForDynamicMode", skip_serializing_if = "Option::is_none")]
    pub has_properties_for_dynamic_mode: Option<bool>,
    #[serde(rename = "isSink", skip_serializing_if = "Option::is_none")]
    pub is_sink: Option<bool>,
    #[serde(rename = "isSource", skip_serializing_if = "Option::is_none")]
    pub is_source: Option<bool>,
    #[serde(rename = "supportsDynamicMode", skip_serializing_if = "Option::is_none")]
    pub supports_dynamic_mode: Option<bool>,
    #[serde(rename = "attachedAdjustID", skip_serializing_if = "Option::is_none")]
    pub attached_adjust_id: Option<String>,
    #[serde(rename = "attachedSpecID", skip_serializing_if = "Option::is_none")]
    pub attached_spec_id: Option<String>,
    #[serde(rename = "calculated", skip_serializing_if = "Option::is_none")]
    pub calculated: Option<bool>,
    #[serde(rename = "debugMode", skip_serializing_if = "Option::is_none")]
    pub debug_mode: Option<bool>,
    #[serde(rename = "lastUpdated", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    #[serde(rename = "preferredFlashAlgorithmTag", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub preferred_flash_algorithm_tag: Option<Option<String>>,
    #[serde(rename = "dynamicMode", skip_serializing_if = "Option::is_none")]
    pub dynamic_mode: Option<bool>,
    #[serde(rename = "fileDatabaseProvider", skip_serializing_if = "Option::is_none")]
    pub file_database_provider: Option<String>,
    #[serde(rename = "dynamicsManager", skip_serializing_if = "Option::is_none")]
    pub dynamics_manager: Option<String>,
    #[serde(rename = "dynamicsOnly", skip_serializing_if = "Option::is_none")]
    pub dynamics_only: Option<bool>,
    #[serde(rename = "visible", skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
    #[serde(rename = "overrideCalculationRoutine", skip_serializing_if = "Option::is_none")]
    pub override_calculation_routine: Option<bool>,
    #[serde(rename = "storeDetailedDebugReport", skip_serializing_if = "Option::is_none")]
    pub store_detailed_debug_report: Option<bool>,
    #[serde(rename = "detailedDebugReport", skip_serializing_if = "Option::is_none")]
    pub detailed_debug_report: Option<String>,
    #[serde(rename = "isFunctional", skip_serializing_if = "Option::is_none")]
    pub is_functional: Option<bool>,
    #[serde(rename = "availableCompounds", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub available_compounds: Option<Option<serde_json::Value>>,
    #[serde(rename = "utilityPlugins", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub utility_plugins: Option<Option<serde_json::Value>>,
    #[serde(rename = "propertyPackages", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub property_packages: Option<Option<serde_json::Value>>,
    #[serde(rename = "availablePropertyPackages", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub available_property_packages: Option<Option<serde_json::Value>>,
    #[serde(rename = "reactions", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub reactions: Option<Option<serde_json::Value>>,
    #[serde(rename = "reactionSets", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub reaction_sets: Option<Option<serde_json::Value>>,
    #[serde(rename = "charts", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub charts: Option<Option<serde_json::Value>>,
    #[serde(rename = "supressMessages", skip_serializing_if = "Option::is_none")]
    pub supress_messages: Option<bool>,
    #[serde(rename = "solved", skip_serializing_if = "Option::is_none")]
    pub solved: Option<bool>,
    #[serde(rename = "errorMessage", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "annotation", skip_serializing_if = "Option::is_none")]
    pub annotation: Option<String>,
    #[serde(rename = "isAdjustAttached", skip_serializing_if = "Option::is_none")]
    pub is_adjust_attached: Option<bool>,
    #[serde(rename = "isSpecAttached", skip_serializing_if = "Option::is_none")]
    pub is_spec_attached: Option<bool>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "attachedUtilities", skip_serializing_if = "Option::is_none")]
    pub attached_utilities: Option<String>,
    #[serde(rename = "componentDescription", skip_serializing_if = "Option::is_none")]
    pub component_description: Option<String>,
    #[serde(rename = "canUsePreviousResults", skip_serializing_if = "Option::is_none")]
    pub can_use_previous_results: Option<bool>,
    #[serde(rename = "userDefinedChartNames", skip_serializing_if = "Option::is_none")]
    pub user_defined_chart_names: Option<bool>,
    #[serde(rename = "isDirty", skip_serializing_if = "Option::is_none")]
    pub is_dirty: Option<bool>,
    #[serde(rename = "externalSolverID", skip_serializing_if = "Option::is_none")]
    pub external_solver_id: Option<String>,
    #[serde(rename = "externalSolverConfigData", skip_serializing_if = "Option::is_none")]
    pub external_solver_config_data: Option<String>,
    #[serde(rename = "fixOnHeat", skip_serializing_if = "Option::is_none")]
    pub fix_on_heat: Option<bool>,
    #[serde(rename = "calculationMode", skip_serializing_if = "Option::is_none")]
    pub calculation_mode: Option<models::HeaterCalculationModeEnum>,
    #[serde(rename = "flowsheetOwner", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub flowsheet_owner: Option<Option<i32>>,
    #[serde(rename = "propertyPackage", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub property_package: Option<Option<i32>>,
    #[serde(rename = "accumulationStream", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub accumulation_stream: Option<Option<i32>>,
}

impl PatchedHeater {
    pub fn new() -> PatchedHeater {
        PatchedHeater {
            simulation_object_id: None,
            x: None,
            y: None,
            properties: None,
            graphic_object: None,
            connector_port: None,
            component_name: None,
            object_class: None,
            dynamics_spec: None,
            spec_var_type: None,
            adjust_var_type: None,
            has_properties_for_dynamic_mode: None,
            is_sink: None,
            is_source: None,
            supports_dynamic_mode: None,
            attached_adjust_id: None,
            attached_spec_id: None,
            calculated: None,
            debug_mode: None,
            last_updated: None,
            preferred_flash_algorithm_tag: None,
            dynamic_mode: None,
            file_database_provider: None,
            dynamics_manager: None,
            dynamics_only: None,
            visible: None,
            override_calculation_routine: None,
            store_detailed_debug_report: None,
            detailed_debug_report: None,
            is_functional: None,
            available_compounds: None,
            utility_plugins: None,
            property_packages: None,
            available_property_packages: None,
            reactions: None,
            reaction_sets: None,
            charts: None,
            supress_messages: None,
            solved: None,
            error_message: None,
            message: None,
            annotation: None,
            is_adjust_attached: None,
            is_spec_attached: None,
            name: None,
            attached_utilities: None,
            component_description: None,
            can_use_previous_results: None,
            user_defined_chart_names: None,
            is_dirty: None,
            external_solver_id: None,
            external_solver_config_data: None,
            fix_on_heat: None,
            calculation_mode: None,
            flowsheet_owner: None,
            property_package: None,
            accumulation_stream: None,
        }
    }
}

