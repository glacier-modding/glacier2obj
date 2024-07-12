use serde::Deserialize;
use std::{fs, io::{self, Write}};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrimsJson {
    pub entities: Vec<EntityHashPair>,
}

impl PrimsJson {
    pub fn build_from_prims_file(prims_json_file: String) -> PrimsJson {
        println!("Building PrimsJson from prims file: {}", prims_json_file);
        io::stdout().flush().unwrap();
        let prims_json_string = fs::read_to_string(prims_json_file.as_str())
        .expect("Should have been able to read the file");
        return PrimsJson::build_from_prims_json_string(prims_json_string)
    }

    pub fn build_from_prims_json_string(prims_json_string: String) -> PrimsJson {
        return serde_json::from_str(&prims_json_string).expect("JSON was not well-formatted");
    }

    pub fn output_prims(&mut self) {
        for entity in &self.entities {
            println!("Entity Instance:");
            println!(" Hash:     {}", entity.prim_hash);
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
pub struct EntityHashPair {
    pub prim_hash: String,
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
