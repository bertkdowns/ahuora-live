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
pub struct PropertySet {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "ContainedProperties")]
    pub contained_properties: Vec<models::PropertyInfo>,
}

impl PropertySet {
    pub fn new(name: String, contained_properties: Vec<models::PropertyInfo>) -> PropertySet {
        PropertySet {
            name,
            contained_properties,
        }
    }
}

