use openapi::apis::core_api::core_propertyinfo_partial_update;
use openapi::apis::configuration::Configuration;
use openapi::models::{property_info, PatchedPropertyInfo};
use serde_json::Value;

#[tokio::main]
async fn main() {

    println!("Hello, world!");

    let mut configuration = Configuration::new();
    configuration.base_path = "http://localhost:8001".to_owned();

    let id = 12;
    let mut property_info = PatchedPropertyInfo::new();
    property_info.value = Some(Some(Value::String("new value".to_string())));
    core_propertyinfo_partial_update(&configuration, id, Some(property_info)).await;
    
}
