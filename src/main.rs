use openapi::apis::core_api::core_propertyinfo_partial_update;
use openapi::apis::unitops_api::unitops_unitops_list;
use openapi::apis::unitops_api::unitops_materialstreams_list;
use openapi::apis::configuration::Configuration;
use openapi::models::MaterialStream;
use openapi::models:: PatchedPropertyInfo;
use openapi::models::UnitOp;
use serde_json::Value;
use std::collections::HashMap;
use std::io;
use std::error::Error;
use csv;
use serde::Deserialize;
use std::fs::File;





type Location = String;
type Measurement = String;
type StateMap = HashMap<Location,HashMap<Measurement,PropertyState>>;

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

struct SensorDefinition {
    location: &'static str, // The sensor provides it's location
    measurement: &'static str, // The sensor provides this measurement
    unitop: &'static str, // E.g PUMP-0
    propkey: &'static str, // E.g "PROP_HX_TEMP"
}


#[derive(Debug)]
struct PropertyState {
    property_id: i32,
    value: Option<f64>,
}

const FLOWSHEET_ID: i32 = 0;

const SENSOR_DEFINITIONS: &[SensorDefinition] = &[
    SensorDefinition {
        location: "PUMP-0",
        measurement: "PROP_HX_TEMP",
        unitop: "PUMP-0",
        propkey: "PROP_HX_TEMP",
    },
];

struct Flowsheet {
    unitops: Vec<UnitOp>,
    materialstreams: Vec<MaterialStream>,
}
impl Flowsheet{
    async fn new(config: &Configuration) -> Result<Flowsheet, Box<dyn Error>> {
        let unitops = unitops_unitops_list(config, FLOWSHEET_ID).await?;
        let materialstreams = unitops_materialstreams_list(config, FLOWSHEET_ID).await?;
        Ok(Flowsheet { unitops, materialstreams })
    } 

    async fn get_property_id(&self, unitop: &str, propkey: &str ) -> Option<i32> {
        // get the unit ops with the unitops_list query, and iterate through them to find the one with the correct name
        for unitop_info in self.unitops.iter() {
            if unitop_info.name == unitop {
                let properties = unitop_info.properties.as_ref().unwrap();
                // get the properties of the unit op
                for property in properties.contained_properties.iter() {
                    if property.prop_key.as_ref().unwrap() == propkey {
                        return Some(property.id);
                    }
                }
            }
        }
        // If not a unit op, do the same thing with material streams
        for materialstream_info in self.materialstreams.iter() {
            if &materialstream_info.name == unitop {
                // get the properties of the unit op
                for property in materialstream_info.properties.as_ref().unwrap().contained_properties.iter() {
                    if property.prop_key.as_ref().unwrap() == propkey {
                        return Some(property.id);
                    }
                }
            }
        }
        return None;
    }
}






async fn initialise_state(config: &Configuration) -> Result<StateMap, Box<dyn Error>> {
    let mut recent_values: StateMap = HashMap::new();

    // Get the flowsheet
    let flowsheet = Flowsheet::new(config).await?;

    // Iterate through sensor definitions, requesting the latest value for each
    for sensor_definition in SENSOR_DEFINITIONS {
        // Get the property id for the sensor (using the unitop name and propkey)
        let property_id = match flowsheet.get_property_id(sensor_definition.unitop, sensor_definition.propkey).await {
            Some(id) => id,
            None => {
                println!("Property not found: {} {}", sensor_definition.unitop, sensor_definition.propkey);
                continue;
            }
        };

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

    return Ok(recent_values);
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    println!("Hello, world!");

    let mut configuration = Configuration::new();
    configuration.base_path = "http://localhost:8001".to_owned();

    let mut recent_values: StateMap = initialise_state(&configuration).await?;

    // Create a CSV parser that reads data from a file
    let file_path =  "../query_data.csv";
    let mut rdr = csv::Reader::from_path(file_path)?;

    for result in rdr.deserialize() {
        let record: Record = result?;
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
    }

    println!("{:?}", recent_values);

    

    let id = 12;
    
    

    return Ok(());
}


