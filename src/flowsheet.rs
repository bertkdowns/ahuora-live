use openapi::apis::unitops_api::unitops_unitops_list;
use openapi::apis::unitops_api::unitops_materialstreams_list;
use openapi::apis::configuration::Configuration;
use openapi::models::MaterialStream;
use openapi::models::UnitOp;
use std::error::Error;

pub struct Flowsheet {
    unitops: Vec<UnitOp>,
    materialstreams: Vec<MaterialStream>,
    id: i32,
}

impl Flowsheet{
    pub async fn new(config: &Configuration,id:i32) -> Result<Flowsheet, Box<dyn Error>> {
        let unitops = match unitops_unitops_list(config, id).await {
            Ok(unitops) => unitops,
            Err(e) => {
                eprintln!("Error getting unitops: {}", e);
                return Err(Box::new(e));
            }
        };
        let materialstreams = match unitops_materialstreams_list(config, id).await {
            Ok(materialstreams) => materialstreams,
            Err(e) => {
                eprintln!("Error getting materialstreams: {}", e);
                return Err(Box::new(e));
            }
        };
        Ok(Flowsheet { unitops, materialstreams,id })
    } 

    pub async fn get_property_id(&self, unitop: &str, propkey: &str ) -> Option<i32> {
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