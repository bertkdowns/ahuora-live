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
pub struct PatchedGrouping {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "_groupGraphic", skip_serializing_if = "Option::is_none")]
    pub _group_graphic: Option<i32>,
    #[serde(rename = "x", skip_serializing_if = "Option::is_none")]
    pub x: Option<String>,
    #[serde(rename = "y", skip_serializing_if = "Option::is_none")]
    pub y: Option<String>,
    #[serde(rename = "_groupName", skip_serializing_if = "Option::is_none")]
    pub _group_name: Option<String>,
    #[serde(rename = "_groupColor", skip_serializing_if = "Option::is_none")]
    pub _group_color: Option<Vec<String>>,
    #[serde(rename = "_groupType", skip_serializing_if = "Option::is_none")]
    pub _group_type: Option<models::GroupTypeEnum>,
    #[serde(rename = "_detailLevel", skip_serializing_if = "Option::is_none")]
    pub _detail_level: Option<i32>,
    #[serde(rename = "_unitOperations", skip_serializing_if = "Option::is_none")]
    pub _unit_operations: Option<Vec<i32>>,
}

impl PatchedGrouping {
    pub fn new() -> PatchedGrouping {
        PatchedGrouping {
            id: None,
            _group_graphic: None,
            x: None,
            y: None,
            _group_name: None,
            _group_color: None,
            _group_type: None,
            _detail_level: None,
            _unit_operations: None,
        }
    }
}

