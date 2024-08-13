// Defines the structure of the input data
use super::types::{CalculatedProperty, SensorDefinition};

pub const SENSOR_DEFINITIONS: &[SensorDefinition] = &[SensorDefinition {
    location: "Wall Plug",
    measurement: "current_power",
    unitop: "my_pump",
    propkey: "PROP_PU_3", // Power Required Property
}];

pub const CALCULATED_PROPERTIES: [CalculatedProperty;2] = [
    CalculatedProperty {
        unitop: "pump_outlet",
        propkey: "PROP_MS_1", // Pressure
        property_id: 0,
        value: 0.0,
    },
    CalculatedProperty {
        unitop: "pump_outlet",
        propkey: "PROP_MS_0", // Temperature
        property_id: 0,
        value: 0.0,
    },
];
