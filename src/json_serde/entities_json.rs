use serde::Deserialize;
use std::{fs, io::{self, Write}};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntitiesJson {
    pub entities: Vec<EntityHashPair>,
}

impl EntitiesJson {
    pub fn get_brick_tblu_hashes(brick_tblu_message_strings: Vec<String>) -> Vec<String> {
        let mut brick_tblu_hashes: Vec<String> = Vec::new();
        for brick_tblu_message_string in brick_tblu_message_strings {
            let brick_message: BrickMessage = serde_json::from_str(&brick_tblu_message_string).expect("Error parsing scene hash.");
            brick_tblu_hashes.push(brick_message.brick_hash);
        }
        return brick_tblu_hashes;
    }

    pub fn build_from_prims_file(prims_json_file: String) -> EntitiesJson {
        println!("Building PrimsJson from prims file: {}", prims_json_file);
        io::stdout().flush().unwrap();
        let prims_json_string = fs::read_to_string(prims_json_file.as_str())
        .expect("Should have been able to read the file");
        return EntitiesJson::build_from_prims_json_string(prims_json_string)
    }

    pub fn build_from_prims_json_string(prims_json_string: String) -> EntitiesJson {
        return serde_json::from_str(&prims_json_string).expect("JSON was not well-formatted");
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrickMessage {
    pub brick_hash: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityHashPair {
    pub hash: String,
    pub entity: Entity,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entity {
    pub id: String,
    pub name: Option<String>,
    pub position: Vec3,
    pub rotation: Rotation,
    pub scale: Scale,
}

#[derive(Debug, Deserialize)]
#[serde()]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rotation {
    pub yaw: f64,
    pub pitch: f64,
    pub roll: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Scale {
    #[serde(rename = "type")]
    pub r#type: String,
    pub data: Vec3,
}
