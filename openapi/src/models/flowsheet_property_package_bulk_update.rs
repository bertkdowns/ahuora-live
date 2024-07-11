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
pub struct FlowsheetPropertyPackageBulkUpdate {
    #[serde(rename = "propertyPackages")]
    pub property_packages: Vec<i32>,
}

impl FlowsheetPropertyPackageBulkUpdate {
    pub fn new(property_packages: Vec<i32>) -> FlowsheetPropertyPackageBulkUpdate {
        FlowsheetPropertyPackageBulkUpdate {
            property_packages,
        }
    }
}

