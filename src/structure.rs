// Defines the structure of the input data
use super::types::{CalculatedProperty, SensorDefinition};

pub const FLOWSHEET_ID: i32 = 2;

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
        display_name: "Pump Outlet Pressure",
        ..CalculatedProperty::default_vals()
    },
    CalculatedProperty {
        unitop: "pump_outlet",
        propkey: "PROP_MS_0", // Temperature
        display_name: "Pump Outlet Temperature",
        ..CalculatedProperty::default_vals()
    },
];
