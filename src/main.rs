use openapi::apis::core_api::core_propertyinfo_partial_update;
use openapi::apis::unitops_api::unitops_unitops_list;
use openapi::apis::unitops_api::unitops_materialstreams_list;
use openapi::apis::configuration::Configuration;
use openapi::apis::solve_api::solve_idaes_create;
use openapi::apis::core_api::core_propertyinfo_retrieve;
use openapi::models::MaterialStream;
use openapi::models:: PatchedPropertyInfo;
use openapi::models::SolveRequest;
use openapi::models::UnitOp;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use serde::Deserialize;





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
struct CalculatedProperty {
    unitop: &'static str,
    propkey: &'static str,
    property_id: i32,
    value: f64,
}


#[derive(Debug)]
struct PropertyState {
    property_id: i32,
    value: Option<f64>,
}

const FLOWSHEET_ID: i32 = 12;

const SENSOR_DEFINITIONS: &[SensorDefinition] = &[
    SensorDefinition {
        location: "Wall Plug",
        measurement: "current_power",
        unitop: "my_pump",
        propkey: "PROP_PU_3", // Power Required Property
    },
];

struct Flowsheet {
    unitops: Vec<UnitOp>,
    materialstreams: Vec<MaterialStream>,
}
impl Flowsheet{
    async fn new(config: &Configuration) -> Result<Flowsheet, Box<dyn Error>> {
        let unitops = match unitops_unitops_list(config, FLOWSHEET_ID).await {
            Ok(unitops) => unitops,
            Err(e) => {
                eprintln!("Error getting unitops: {}", e);
                return Err(Box::new(e));
            }
        };
        let materialstreams = match unitops_materialstreams_list(config, FLOWSHEET_ID).await {
            Ok(materialstreams) => materialstreams,
            Err(e) => {
                eprintln!("Error getting materialstreams: {}", e);
                return Err(Box::new(e));
            }
        };
        Ok(Flowsheet { unitops, materialstreams })
    } 

    async fn get_property_id(&self, unitop: &str, propkey: &str ) -> Option<i32> {
        // get the unit ops with the unitops_list query, and iterate through them to find the one with the correct name
        for unitop_info in self.unitops.iter() {
            if unitop_info.component_name.as_ref()? == unitop {
                let properties = unitop_info.properties.as_ref()?;
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
            println!("{:?}", materialstream_info.component_name);
            if materialstream_info.component_name.as_ref()? == unitop {
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






async fn initialise_state(flowsheet: &Flowsheet) -> Result<StateMap, Box<dyn Error>> {
    let mut recent_values: StateMap = HashMap::new();

    // Iterate through sensor definitions, requesting the latest value for each
    for sensor_definition in SENSOR_DEFINITIONS {
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

    println!("Hello, world!");

    let mut configuration = Configuration::new();
    configuration.base_path = "http://localhost:8001".to_owned();
    // Get the flowsheet
    let flowsheet = match Flowsheet::new(&configuration).await {
        Ok(flowsheet) => flowsheet,
        Err(e) => {
            println!("Error getting flowsheet: {}", e);
            return Ok(());
        }
    };



    let mut calculated_properties = vec![
        CalculatedProperty {
            unitop: "pump_outlet",
            propkey: "PROP_MS_1", // Pressure
            property_id: 0,
            value: 0.0,
        }, CalculatedProperty {
            unitop: "pump_outlet",
            propkey: "PROP_MS_0", // Temperature
            property_id: 0,
            value: 0.0,
        },
    ];
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


