use serde::{Serialize, Deserialize};
use std::{fs, io::{self, Write}};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntitiesJson {
    pub meshes: Vec<MeshHashesAndEntity>,
    #[serde(rename = "pfBoxes")]
    pub pf_boxes: Vec<PfBox>,
    #[serde(rename = "pfSeedPoints")]
    pub pf_seed_points: Vec<PfSeedPoint>,
}

impl EntitiesJson {
    pub fn build_from_nav_json_file(nav_json_file: String) -> EntitiesJson {
        println!("Loading scene from nav.json file: {}", nav_json_file);
        io::stdout().flush().unwrap();
        let nav_json_string = fs::read_to_string(nav_json_file.as_str())
            .expect("Error reading nav.json file");
        return EntitiesJson::build_from_nav_json_string(nav_json_string);
    }

    pub fn build_from_nav_json_string(nav_json_string: String) -> EntitiesJson {
        return serde_json::from_str(&nav_json_string).expect("JSON was not well-formatted");
    }

    pub fn output_entities(&mut self) {
        for entity in &self.meshes {
            println!("Entity Instance:");
            println!(" Aloc Hash:     {}", entity.aloc_hash);
            println!(" Prim Hash:     {}", entity.prim_hash);
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
pub struct MeshHashesAndEntity {
    pub aloc_hash: String,
    pub prim_hash: String,
    pub entity: Aloc,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Aloc {
    pub id: String,
    pub name: Option<String>,
    pub tblu: Option<String>,
    pub position: Vec3,
    pub rotation: Rotation,
    pub scale: Scale,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PfBox {
    pub id: String,
    pub position: Vec3,
    pub rotation: Rotation,
    #[serde(rename = "type")]
    pub r#type: Type,
    pub scale: Scale,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PfSeedPoint {
    pub id: String,
    pub position: Vec3,
    pub rotation: Rotation,
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Type {
    #[serde(rename = "type")]
    pub r#type: String,
    pub data: String,
}
