use openapi::apis::core_api::core_propertyinfo_partial_update;
use openapi::apis::configuration::Configuration;
use openapi::models:: PatchedPropertyInfo;
use serde_json::Value;
use std::collections::HashMap;
use std::io;
use std::error::Error;
use csv;
use serde::Deserialize;
use std::fs::File;





type Location = String;
type Measurement = String;

#[derive(Debug, Deserialize)]
#[serde()]
struct Record {
    // result: f64,
    // table: f64,
    // _start: String,
    // _stop: String,
    // _time: String,
    _value: f64,
    // _field: String,
    _measurement: Measurement,
    location: Location,
    // name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    println!("Hello, world!");

    let mut recent_values: HashMap<Location,HashMap<Measurement,f64>> = HashMap::new();

    // Create a CSV parser that reads data from a file
    let file_path =  "../query_data.csv";
    let mut rdr = csv::Reader::from_path(file_path)?;

    for result in rdr.deserialize() {
        let record: Record = result?;
        let location = record.location;
        let measurement = record._measurement;
        let value = record._value;

        // Insert the value into the hashmap
        let location_map = recent_values.entry(location).or_insert(HashMap::new());
        location_map.insert(measurement, value);
    }

    println!("{:?}", recent_values);

    let mut configuration = Configuration::new();
    configuration.base_path = "http://localhost:8001".to_owned();

    let id = 12;
    let mut property_info = PatchedPropertyInfo::new();
    property_info.value = Some(Some(Value::String("new value".to_string())));
    core_propertyinfo_partial_update(&configuration, id, Some(property_info)).await;

    return Ok(());
}
