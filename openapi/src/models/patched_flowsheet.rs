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
pub struct PatchedFlowsheet {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "buildVersion", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub build_version: Option<Option<String>>,
    #[serde(rename = "buildDate", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub build_date: Option<Option<String>>,
    #[serde(rename = "savedOn", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub saved_on: Option<Option<String>>,
}

impl PatchedFlowsheet {
    pub fn new() -> PatchedFlowsheet {
        PatchedFlowsheet {
            id: None,
            name: None,
            build_version: None,
            build_date: None,
            saved_on: None,
        }
    }
}

