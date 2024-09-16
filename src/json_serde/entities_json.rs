use serde::{Serialize, Deserialize};
use std::{fs, io::{self, Write}};

use crate::json_serde::entities_json;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntitiesJson {
    pub entities: Vec<EntityHashPair>,
}

impl EntitiesJson {
    pub fn build_from_alocs_file(alocs_json_file: String) -> EntitiesJson {
        println!("Building AlocsJson from alocs file: {}", alocs_json_file);
        io::stdout().flush().unwrap();
        let alocs_json_string = fs::read_to_string(alocs_json_file.as_str())
        .expect("Should have been able to read the file");
        return EntitiesJson::build_from_alocs_json_string(alocs_json_string)
    }

    pub fn write_to_alocs_file(&mut self, alocs_json_file: String) {
        println!("Writing AlocsJson to alocs file: {}", alocs_json_file);
        io::stdout().flush().unwrap();
        let alocs_json_string = serde_json::to_string(&self.entities).unwrap();
        fs::write(alocs_json_file, alocs_json_string.as_str())
        .expect("Should have been able to write to the file");
        
    }

    pub fn build_from_alocs_json_string(alocs_json_string: String) -> EntitiesJson {
        return serde_json::from_str(&alocs_json_string).expect("JSON was not well-formatted");
    }

    pub fn output_entities(&mut self) {
        for entity in &self.entities {
            println!("Entity Instance:");
            println!(" Hash:     {}", entity.hash);
            println!(" ID:       {}", entity.entity.id);
            println!(" Name:     {}", entity.entity.name.clone().unwrap_or(String::from("")));
            println!(" Position: {:?}", entity.entity.position);
            println!(" Rotation: {:?}", entity.entity.rotation);
            println!(" Scale:    {:?}", entity.entity.scale);
            io::stdout().flush().unwrap();
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrickMessage {
    pub brick_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityHashPair {
    pub hash: String,
    pub entity: Entity,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entity {
    pub id: String,
    pub name: Option<String>,
    pub position: Vec3,
    pub rotation: Rotation,
    pub scale: Scale,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde()]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rotation {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Scale {
    #[serde(rename = "type")]
    pub r#type: String,
    pub data: Vec3,
}
