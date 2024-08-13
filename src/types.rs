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
    pub property_id: i32,
    pub value: f64,
}

#[derive(Debug)]
pub struct PropertyState {
    pub property_id: i32,
    pub value: Option<f64>,
}
