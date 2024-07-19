/*
 * Your Project API
 *
 * Your project description
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};


#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GraphicObject {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(rename = "flippedH", skip_serializing_if = "Option::is_none")]
    pub flipped_h: Option<bool>,
    #[serde(rename = "flippedV", skip_serializing_if = "Option::is_none")]
    pub flipped_v: Option<bool>,
    #[serde(rename = "x", skip_serializing_if = "Option::is_none")]
    pub x: Option<String>,
    #[serde(rename = "y", skip_serializing_if = "Option::is_none")]
    pub y: Option<String>,
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<String>,
    #[serde(rename = "width", skip_serializing_if = "Option::is_none")]
    pub width: Option<String>,
    #[serde(rename = "rotation", skip_serializing_if = "Option::is_none")]
    pub rotation: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<models::StatusEnum>,
}

impl GraphicObject {
    pub fn new(id: i32) -> GraphicObject {
        GraphicObject {
            id,
            tag: None,
            flipped_h: None,
            flipped_v: None,
            x: None,
            y: None,
            height: None,
            width: None,
            rotation: None,
            status: None,
        }
    }
}

