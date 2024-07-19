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
/// PressureDropCorrelationEnum : * `0` - Beggsbrill * `1` - Lockhartmartinelli * `2` - Petalasaziz
/// * `0` - Beggsbrill * `1` - Lockhartmartinelli * `2` - Petalasaziz
#[repr(i64)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize_repr, Deserialize_repr)]
pub enum PressureDropCorrelationEnum {
    Variant0 = 0,
    Variant1 = 1,
    Variant2 = 2,

}

impl ToString for PressureDropCorrelationEnum {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0 => String::from("0"),
            Self::Variant1 => String::from("1"),
            Self::Variant2 => String::from("2"),
        }
    }
}
impl Default for PressureDropCorrelationEnum {
    fn default() -> PressureDropCorrelationEnum {
        Self::Variant0
    }
}

