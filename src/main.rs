use openapi::apis::core_api::core_propertyinfo_partial_update;
use openapi::apis::configuration::Configuration;
use openapi::apis::solve_api::solve_idaes_create;
use openapi::apis::core_api::core_propertyinfo_retrieve;
use openapi::models:: PatchedPropertyInfo;
use openapi::models::SolveRequest;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use flowsheet::Flowsheet;
use color_eyre::eyre::Result;

mod flowsheet;
mod structure;
mod types;

use types::{Measurement, Location, Record, PropertyState};

type StateMap = HashMap<Location,HashMap<Measurement,PropertyState>>;


const FLOWSHEET_ID: i32 = 2;



async fn initialise_state(flowsheet: &Flowsheet) -> Result<StateMap, Box<dyn Error>> {
    let mut recent_values: StateMap = HashMap::new();

    // Iterate through sensor definitions, requesting the latest value for each
    for sensor_definition in structure::SENSOR_DEFINITIONS {
        // Get the property id for the sensor (using the unitop name and propkey)
        let property_id = match flowsheet.get_property_id(sensor_definition.unitop, sensor_definition.propkey).await {
            Some(id) => Some(id),
            None => {
                eprintln!("Property not found: {} {}", sensor_definition.unitop, sensor_definition.propkey);
                continue;
            }
        }.ok_or_else(|| format!("Property not found: {} {}", sensor_definition.unitop, sensor_definition.propkey))?;
        println!("Property ID: {}", property_id);
        // Insert the value into the hashmap
        let location_map = recent_values
            .entry(sensor_definition.location.to_string())
            .or_insert(HashMap::new());
        let property_state = PropertyState {
            property_id: property_id,
            value: None,
        };
        location_map.insert(sensor_definition.measurement.to_string(), property_state);
    }

    Ok(recent_values)
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    color_eyre::install()?;
    println!("Hello, world!");

    let mut configuration = Configuration::new();
    configuration.base_path = "http://localhost:8001".to_owned();
    // Get the flowsheet
    let flowsheet = match Flowsheet::new(&configuration,FLOWSHEET_ID).await {
        Ok(flowsheet) => flowsheet,
        Err(e) => {
            println!("Error getting flowsheet: {}", e);
            return Ok(());
        }
    };



    let mut calculated_properties = structure::CALCULATED_PROPERTIES.clone();
    for calculated_property in calculated_properties.iter_mut() {
        calculated_property.property_id = match flowsheet.get_property_id(calculated_property.unitop, calculated_property.propkey).await {
            Some(id) => id,
            None => {
                println!("Property not found: {} {}", calculated_property.unitop, calculated_property.propkey);
                continue;
            }
        };
    }

    
    let mut recent_values: StateMap = match initialise_state(&flowsheet).await {
        Ok(state) => state,
        Err(e) => {
            println!("Error initialising state: {}", e);
            return Ok(());
        }
    };

    // Create a CSV parser that reads data from a file
    let file_path =  "./example_data.csv"; // or for real data use"../query_data.csv";
    let mut rdr = csv::Reader::from_path(file_path)?;

    for result in rdr.deserialize() {
        let record: Record = match result {
            Ok(record) => record,
            Err(e) => {
                println!("Error reading csv record: {}", e);
                continue;
            }
        };
        let location = record.location;
        let measurement = record._measurement;
        let value = record._value;

        // Insert the value into the hashmap
        let location_map = match recent_values.get_mut(&location) {
            Some(map) => map,
            None => {
                println!("Location not found: {}", location);
                continue;
            }
        };
        let property = match location_map.get_mut (&measurement) {
            Some(property) => property,
            None => {
                println!("Measurement not found: {}", measurement);
                continue;
            }
        };
        // Update the value
        property.value = Some(value);
        let mut property_info = PatchedPropertyInfo::new();
        property_info.value = Some(Some(Value::String(value.to_string())));
        match core_propertyinfo_partial_update(&configuration, property.property_id, Some(property_info)).await {
            Ok(_) => println!("Updated property: {} {}", location, measurement),
            Err(e) => println!("Error updating property: {} {} \n {}", location, measurement, e),
        }
        // solve with updated value
        match solve_idaes_create(&configuration, SolveRequest {
            flowsheet_id: FLOWSHEET_ID,
            debug: Some(true),
            require_variables_fixed: Some(true),
        }).await {
            Ok(_) => println!("Solved with updated value: {} {}", location, measurement),
            Err(_) => println!("Error solving with updated value: {} {}", location, measurement),
        }
        
        // get the updated values that we care about
        for calculated_property in calculated_properties.iter_mut() {
            let property_id = calculated_property.property_id;
            let property_info = match core_propertyinfo_retrieve(&configuration, property_id).await {
                Ok(info) => info,
                Err(e) => {
                    println!("Error getting property info: {}", e);
                    continue;
                }
            };
            let value = match property_info.value.unwrap().unwrap(){
                Value::Number(n) => n.as_f64().unwrap(),
                _ => {
                    println!("Unexpected value type");
                    continue;
                }
            
            };
            calculated_property.value = value;
        }
        println!("{:?}", calculated_properties);
    }
    println!("{:?}", recent_values);
    


    return Ok(());
}


