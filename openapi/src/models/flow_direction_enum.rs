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

/// FlowDirectionEnum : * `0` - Countercurrent * `1` - Cocurrent
/// * `0` - Countercurrent * `1` - Cocurrent
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FlowDirectionEnum {
    #[serde(rename = "0")]
    Variant0,
    #[serde(rename = "1")]
    Variant1,

}

impl std::fmt::Display for FlowDirectionEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Variant0 => write!(f, "0"),
            Self::Variant1 => write!(f, "1"),
        }
    }
}

impl Default for FlowDirectionEnum {
    fn default() -> FlowDirectionEnum {
        Self::Variant0
    }
}

