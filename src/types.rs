use serde::Deserialize;

pub type Location = String;
pub type Measurement = String;

#[derive(Debug, Deserialize)]
#[serde()]
pub struct Record {
    // result: f64,
    // table: f64,
    // _start: String,
    // _stop: String,
    // _time: String,
    pub _value: f64,
    // _field: String,
    pub _measurement: Measurement,
    pub location: Location,
    // name: String,
}

pub struct SensorDefinition {
    pub location: &'static str, // The sensor provides it's location
    pub measurement: &'static str, // The sensor provides this measurement
    pub unitop: &'static str, // E.g PUMP-0
    pub propkey: &'static str, // E.g "PROP_HX_TEMP"
}

#[derive(Debug,Clone)]
pub struct CalculatedProperty {
    pub unitop: &'static str,
    pub propkey: &'static str,
    pub display_name: &'static str,
    pub property_id: i32,
    pub value: f64,
}

impl CalculatedProperty {
    // Cant use default::Default because needs to be a const function.
    pub const fn default_vals() -> Self {
        Self {
            unitop: "",
            propkey: "",
            display_name: "",
            property_id: 0,
            value: 0.0,
        }
    }
}

#[derive(Debug)]
pub struct PropertyState {
    pub property_id: i32,
    pub value: Option<f64>,
}
