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

/// FloatingTableAmountBasisEnum : * `Molar_Fractions` - Molar Fractions * `Mass_Fractions` - Mass Fractions * `Volumetric_Fractions` - Volumetric Fractions * `Molar_Flows` - Molar Flows * `Mass_Flows` - Mass Flows * `Volumetric_Flows` - Volumetric Flows * `DefaultBasis` - Defaultbasis
/// * `Molar_Fractions` - Molar Fractions * `Mass_Fractions` - Mass Fractions * `Volumetric_Fractions` - Volumetric Fractions * `Molar_Flows` - Molar Flows * `Mass_Flows` - Mass Flows * `Volumetric_Flows` - Volumetric Flows * `DefaultBasis` - Defaultbasis
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FloatingTableAmountBasisEnum {
    #[serde(rename = "Molar_Fractions")]
    MolarFractions,
    #[serde(rename = "Mass_Fractions")]
    MassFractions,
    #[serde(rename = "Volumetric_Fractions")]
    VolumetricFractions,
    #[serde(rename = "Molar_Flows")]
    MolarFlows,
    #[serde(rename = "Mass_Flows")]
    MassFlows,
    #[serde(rename = "Volumetric_Flows")]
    VolumetricFlows,
    #[serde(rename = "DefaultBasis")]
    DefaultBasis,

}

impl std::fmt::Display for FloatingTableAmountBasisEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::MolarFractions => write!(f, "Molar_Fractions"),
            Self::MassFractions => write!(f, "Mass_Fractions"),
            Self::VolumetricFractions => write!(f, "Volumetric_Fractions"),
            Self::MolarFlows => write!(f, "Molar_Flows"),
            Self::MassFlows => write!(f, "Mass_Flows"),
            Self::VolumetricFlows => write!(f, "Volumetric_Flows"),
            Self::DefaultBasis => write!(f, "DefaultBasis"),
        }
    }
}

impl Default for FloatingTableAmountBasisEnum {
    fn default() -> FloatingTableAmountBasisEnum {
        Self::MolarFractions
    }
}

