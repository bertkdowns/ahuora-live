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
pub struct Grouping {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "_groupGraphic")]
    pub _group_graphic: i32,
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
    #[serde(rename = "_unitOperations")]
    pub _unit_operations: Vec<i32>,
}

impl Grouping {
    pub fn new(id: i32, _group_graphic: i32, _unit_operations: Vec<i32>) -> Grouping {
        Grouping {
            id,
            _group_graphic,
            x: None,
            y: None,
            _group_name: None,
            _group_color: None,
            _group_type: None,
            _detail_level: None,
            _unit_operations,
        }
    }
}

