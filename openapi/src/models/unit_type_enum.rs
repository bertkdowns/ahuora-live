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

/// UnitTypeEnum : * `accel` - Accel * `activity` - Activity * `activityCoefficient` - Activitycoefficient * `area` - Area * `boilingPointTemperature` - Boilingpointtemperature * `cakeresistance` - Cakeresistance * `cinematic_viscosity` - Cinematic Viscosity * `compressibility` - Compressibility * `compressibilityFactor` - Compressibilityfactor * `deltaP` - Deltap * `deltaT` - Deltat * `density` - Density * `diameter` - Diameter * `distance` - Distance * `enthalpy` - Enthalpy * `entropy` - Entropy * `excessEnthalpy` - Excessenthalpy * `excessEntropy` - Excessentropy * `force` - Force * `foulingfactor` - Foulingfactor * `fugacity` - Fugacity * `fugacityCoefficient` - Fugacitycoefficient * `gor` - Gor * `head` - Head * `heat_transf_coeff` - Heat Transf Coeff * `heatCapacityCp` - Heatcapacitycp * `heatCapacityCv` - Heatcapacitycv * `heatflow` - Heatflow * `idealGasHeatCapacity` - Idealgasheatcapacity * `jouleThomsonCoefficient` - Joulethomsoncoefficient * `kvalue` - Kvalue * `logFugacityCoefficient` - Logfugacitycoefficient * `logKvalue` - Logkvalue * `mass` - Mass * `mass_conc` - Mass Conc * `massflow` - Massflow * `massfraction` - Massfraction * `mediumresistance` - Mediumresistance * `meltingTemperature` - Meltingtemperature * `molar_conc` - Molar Conc * `molar_enthalpy` - Molar Enthalpy * `molar_entropy` - Molar Entropy * `molar_volume` - Molar Volume * `molarflow` - Molarflow * `molarfraction` - Molarfraction * `molecularWeight` - Molecularweight * `pressure` - Pressure * `reac_rate` - Reac Rate * `reac_rate_heterog` - Reac Rate Heterog * `spec_vol` - Spec Vol * `speedOfSound` - Speedofsound * `surfaceTension` - Surfacetension * `temperature` - Temperature * `thermalConductivity` - Thermalconductivity * `thermalConductivityOfLiquid` - Thermalconductivityofliquid * `thermalConductivityOfVapor` - Thermalconductivityofvapor * `thickness` - Thickness * `time` - Time * `vaporPressure` - Vaporpressure * `velocity` - Velocity * `viscosity` - Viscosity * `viscosityOfLiquid` - Viscosityofliquid * `viscosityOfVapor` - Viscosityofvapor * `volume` - Volume * `volumetricFlow` - Volumetricflow * `diffusivity` - Diffusivity * `none` - None * `conductance` - Conductance * `percentage` - Percentage * `rpm` - Rpm * `voltage` - Voltage * `ampere` - Ampere * `resistance` - Resistance * `irradiance` - Irradiance * `currentDensity` - Currentdensity
/// * `accel` - Accel * `activity` - Activity * `activityCoefficient` - Activitycoefficient * `area` - Area * `boilingPointTemperature` - Boilingpointtemperature * `cakeresistance` - Cakeresistance * `cinematic_viscosity` - Cinematic Viscosity * `compressibility` - Compressibility * `compressibilityFactor` - Compressibilityfactor * `deltaP` - Deltap * `deltaT` - Deltat * `density` - Density * `diameter` - Diameter * `distance` - Distance * `enthalpy` - Enthalpy * `entropy` - Entropy * `excessEnthalpy` - Excessenthalpy * `excessEntropy` - Excessentropy * `force` - Force * `foulingfactor` - Foulingfactor * `fugacity` - Fugacity * `fugacityCoefficient` - Fugacitycoefficient * `gor` - Gor * `head` - Head * `heat_transf_coeff` - Heat Transf Coeff * `heatCapacityCp` - Heatcapacitycp * `heatCapacityCv` - Heatcapacitycv * `heatflow` - Heatflow * `idealGasHeatCapacity` - Idealgasheatcapacity * `jouleThomsonCoefficient` - Joulethomsoncoefficient * `kvalue` - Kvalue * `logFugacityCoefficient` - Logfugacitycoefficient * `logKvalue` - Logkvalue * `mass` - Mass * `mass_conc` - Mass Conc * `massflow` - Massflow * `massfraction` - Massfraction * `mediumresistance` - Mediumresistance * `meltingTemperature` - Meltingtemperature * `molar_conc` - Molar Conc * `molar_enthalpy` - Molar Enthalpy * `molar_entropy` - Molar Entropy * `molar_volume` - Molar Volume * `molarflow` - Molarflow * `molarfraction` - Molarfraction * `molecularWeight` - Molecularweight * `pressure` - Pressure * `reac_rate` - Reac Rate * `reac_rate_heterog` - Reac Rate Heterog * `spec_vol` - Spec Vol * `speedOfSound` - Speedofsound * `surfaceTension` - Surfacetension * `temperature` - Temperature * `thermalConductivity` - Thermalconductivity * `thermalConductivityOfLiquid` - Thermalconductivityofliquid * `thermalConductivityOfVapor` - Thermalconductivityofvapor * `thickness` - Thickness * `time` - Time * `vaporPressure` - Vaporpressure * `velocity` - Velocity * `viscosity` - Viscosity * `viscosityOfLiquid` - Viscosityofliquid * `viscosityOfVapor` - Viscosityofvapor * `volume` - Volume * `volumetricFlow` - Volumetricflow * `diffusivity` - Diffusivity * `none` - None * `conductance` - Conductance * `percentage` - Percentage * `rpm` - Rpm * `voltage` - Voltage * `ampere` - Ampere * `resistance` - Resistance * `irradiance` - Irradiance * `currentDensity` - Currentdensity
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UnitTypeEnum {
    #[serde(rename = "accel")]
    Accel,
    #[serde(rename = "activity")]
    Activity,
    #[serde(rename = "activityCoefficient")]
    ActivityCoefficient,
    #[serde(rename = "area")]
    Area,
    #[serde(rename = "boilingPointTemperature")]
    BoilingPointTemperature,
    #[serde(rename = "cakeresistance")]
    Cakeresistance,
    #[serde(rename = "cinematic_viscosity")]
    CinematicViscosity,
    #[serde(rename = "compressibility")]
    Compressibility,
    #[serde(rename = "compressibilityFactor")]
    CompressibilityFactor,
    #[serde(rename = "deltaP")]
    DeltaP,
    #[serde(rename = "deltaT")]
    DeltaT,
    #[serde(rename = "density")]
    Density,
    #[serde(rename = "diameter")]
    Diameter,
    #[serde(rename = "distance")]
    Distance,
    #[serde(rename = "enthalpy")]
    Enthalpy,
    #[serde(rename = "entropy")]
    Entropy,
    #[serde(rename = "excessEnthalpy")]
    ExcessEnthalpy,
    #[serde(rename = "excessEntropy")]
    ExcessEntropy,
    #[serde(rename = "force")]
    Force,
    #[serde(rename = "foulingfactor")]
    Foulingfactor,
    #[serde(rename = "fugacity")]
    Fugacity,
    #[serde(rename = "fugacityCoefficient")]
    FugacityCoefficient,
    #[serde(rename = "gor")]
    Gor,
    #[serde(rename = "head")]
    Head,
    #[serde(rename = "heat_transf_coeff")]
    HeatTransfCoeff,
    #[serde(rename = "heatCapacityCp")]
    HeatCapacityCp,
    #[serde(rename = "heatCapacityCv")]
    HeatCapacityCv,
    #[serde(rename = "heatflow")]
    Heatflow,
    #[serde(rename = "idealGasHeatCapacity")]
    IdealGasHeatCapacity,
    #[serde(rename = "jouleThomsonCoefficient")]
    JouleThomsonCoefficient,
    #[serde(rename = "kvalue")]
    Kvalue,
    #[serde(rename = "logFugacityCoefficient")]
    LogFugacityCoefficient,
    #[serde(rename = "logKvalue")]
    LogKvalue,
    #[serde(rename = "mass")]
    Mass,
    #[serde(rename = "mass_conc")]
    MassConc,
    #[serde(rename = "massflow")]
    Massflow,
    #[serde(rename = "massfraction")]
    Massfraction,
    #[serde(rename = "mediumresistance")]
    Mediumresistance,
    #[serde(rename = "meltingTemperature")]
    MeltingTemperature,
    #[serde(rename = "molar_conc")]
    MolarConc,
    #[serde(rename = "molar_enthalpy")]
    MolarEnthalpy,
    #[serde(rename = "molar_entropy")]
    MolarEntropy,
    #[serde(rename = "molar_volume")]
    MolarVolume,
    #[serde(rename = "molarflow")]
    Molarflow,
    #[serde(rename = "molarfraction")]
    Molarfraction,
    #[serde(rename = "molecularWeight")]
    MolecularWeight,
    #[serde(rename = "pressure")]
    Pressure,
    #[serde(rename = "reac_rate")]
    ReacRate,
    #[serde(rename = "reac_rate_heterog")]
    ReacRateHeterog,
    #[serde(rename = "spec_vol")]
    SpecVol,
    #[serde(rename = "speedOfSound")]
    SpeedOfSound,
    #[serde(rename = "surfaceTension")]
    SurfaceTension,
    #[serde(rename = "temperature")]
    Temperature,
    #[serde(rename = "thermalConductivity")]
    ThermalConductivity,
    #[serde(rename = "thermalConductivityOfLiquid")]
    ThermalConductivityOfLiquid,
    #[serde(rename = "thermalConductivityOfVapor")]
    ThermalConductivityOfVapor,
    #[serde(rename = "thickness")]
    Thickness,
    #[serde(rename = "time")]
    Time,
    #[serde(rename = "vaporPressure")]
    VaporPressure,
    #[serde(rename = "velocity")]
    Velocity,
    #[serde(rename = "viscosity")]
    Viscosity,
    #[serde(rename = "viscosityOfLiquid")]
    ViscosityOfLiquid,
    #[serde(rename = "viscosityOfVapor")]
    ViscosityOfVapor,
    #[serde(rename = "volume")]
    Volume,
    #[serde(rename = "volumetricFlow")]
    VolumetricFlow,
    #[serde(rename = "diffusivity")]
    Diffusivity,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "conductance")]
    Conductance,
    #[serde(rename = "percentage")]
    Percentage,
    #[serde(rename = "rpm")]
    Rpm,
    #[serde(rename = "voltage")]
    Voltage,
    #[serde(rename = "ampere")]
    Ampere,
    #[serde(rename = "resistance")]
    Resistance,
    #[serde(rename = "irradiance")]
    Irradiance,
    #[serde(rename = "currentDensity")]
    CurrentDensity,

}

impl std::fmt::Display for UnitTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Accel => write!(f, "accel"),
            Self::Activity => write!(f, "activity"),
            Self::ActivityCoefficient => write!(f, "activityCoefficient"),
            Self::Area => write!(f, "area"),
            Self::BoilingPointTemperature => write!(f, "boilingPointTemperature"),
            Self::Cakeresistance => write!(f, "cakeresistance"),
            Self::CinematicViscosity => write!(f, "cinematic_viscosity"),
            Self::Compressibility => write!(f, "compressibility"),
            Self::CompressibilityFactor => write!(f, "compressibilityFactor"),
            Self::DeltaP => write!(f, "deltaP"),
            Self::DeltaT => write!(f, "deltaT"),
            Self::Density => write!(f, "density"),
            Self::Diameter => write!(f, "diameter"),
            Self::Distance => write!(f, "distance"),
            Self::Enthalpy => write!(f, "enthalpy"),
            Self::Entropy => write!(f, "entropy"),
            Self::ExcessEnthalpy => write!(f, "excessEnthalpy"),
            Self::ExcessEntropy => write!(f, "excessEntropy"),
            Self::Force => write!(f, "force"),
            Self::Foulingfactor => write!(f, "foulingfactor"),
            Self::Fugacity => write!(f, "fugacity"),
            Self::FugacityCoefficient => write!(f, "fugacityCoefficient"),
            Self::Gor => write!(f, "gor"),
            Self::Head => write!(f, "head"),
            Self::HeatTransfCoeff => write!(f, "heat_transf_coeff"),
            Self::HeatCapacityCp => write!(f, "heatCapacityCp"),
            Self::HeatCapacityCv => write!(f, "heatCapacityCv"),
            Self::Heatflow => write!(f, "heatflow"),
            Self::IdealGasHeatCapacity => write!(f, "idealGasHeatCapacity"),
            Self::JouleThomsonCoefficient => write!(f, "jouleThomsonCoefficient"),
            Self::Kvalue => write!(f, "kvalue"),
            Self::LogFugacityCoefficient => write!(f, "logFugacityCoefficient"),
            Self::LogKvalue => write!(f, "logKvalue"),
            Self::Mass => write!(f, "mass"),
            Self::MassConc => write!(f, "mass_conc"),
            Self::Massflow => write!(f, "massflow"),
            Self::Massfraction => write!(f, "massfraction"),
            Self::Mediumresistance => write!(f, "mediumresistance"),
            Self::MeltingTemperature => write!(f, "meltingTemperature"),
            Self::MolarConc => write!(f, "molar_conc"),
            Self::MolarEnthalpy => write!(f, "molar_enthalpy"),
            Self::MolarEntropy => write!(f, "molar_entropy"),
            Self::MolarVolume => write!(f, "molar_volume"),
            Self::Molarflow => write!(f, "molarflow"),
            Self::Molarfraction => write!(f, "molarfraction"),
            Self::MolecularWeight => write!(f, "molecularWeight"),
            Self::Pressure => write!(f, "pressure"),
            Self::ReacRate => write!(f, "reac_rate"),
            Self::ReacRateHeterog => write!(f, "reac_rate_heterog"),
            Self::SpecVol => write!(f, "spec_vol"),
            Self::SpeedOfSound => write!(f, "speedOfSound"),
            Self::SurfaceTension => write!(f, "surfaceTension"),
            Self::Temperature => write!(f, "temperature"),
            Self::ThermalConductivity => write!(f, "thermalConductivity"),
            Self::ThermalConductivityOfLiquid => write!(f, "thermalConductivityOfLiquid"),
            Self::ThermalConductivityOfVapor => write!(f, "thermalConductivityOfVapor"),
            Self::Thickness => write!(f, "thickness"),
            Self::Time => write!(f, "time"),
            Self::VaporPressure => write!(f, "vaporPressure"),
            Self::Velocity => write!(f, "velocity"),
            Self::Viscosity => write!(f, "viscosity"),
            Self::ViscosityOfLiquid => write!(f, "viscosityOfLiquid"),
            Self::ViscosityOfVapor => write!(f, "viscosityOfVapor"),
            Self::Volume => write!(f, "volume"),
            Self::VolumetricFlow => write!(f, "volumetricFlow"),
            Self::Diffusivity => write!(f, "diffusivity"),
            Self::None => write!(f, "none"),
            Self::Conductance => write!(f, "conductance"),
            Self::Percentage => write!(f, "percentage"),
            Self::Rpm => write!(f, "rpm"),
            Self::Voltage => write!(f, "voltage"),
            Self::Ampere => write!(f, "ampere"),
            Self::Resistance => write!(f, "resistance"),
            Self::Irradiance => write!(f, "irradiance"),
            Self::CurrentDensity => write!(f, "currentDensity"),
        }
    }
}

impl Default for UnitTypeEnum {
    fn default() -> UnitTypeEnum {
        Self::Accel
    }
}

