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

use serde_repr::{Serialize_repr,Deserialize_repr};
/// DefinedTemperatureEnum : * `0` - Hot Fluid * `1` - Cold Fluid
/// * `0` - Hot Fluid * `1` - Cold Fluid
#[repr(i64)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize_repr, Deserialize_repr)]
pub enum DefinedTemperatureEnum {
    Variant0 = 0,
    Variant1 = 1,

}

impl ToString for DefinedTemperatureEnum {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0 => String::from("0"),
            Self::Variant1 => String::from("1"),
        }
    }
}
impl Default for DefinedTemperatureEnum {
    fn default() -> DefinedTemperatureEnum {
        Self::Variant0
    }
}

