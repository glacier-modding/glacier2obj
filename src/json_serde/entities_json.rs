use serde::{Serialize, Deserialize};
use std::{fs, io::{self, Write}};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntitiesJson {
    pub alocs: Vec<AlocHashPair>,
    #[serde(rename = "pfBoxes")]
    pub pf_boxes: Vec<PfBoxHashPair>,
}

impl EntitiesJson {
    pub fn build_from_nav_json_file(nav_json_file: String) -> EntitiesJson {
        println!("Building EntitiesJson from nav.json file: {}", nav_json_file);
        io::stdout().flush().unwrap();
        let nav_json_string = fs::read_to_string(nav_json_file.as_str())
            .expect("Should have been able to read the file");
        return EntitiesJson::build_from_nav_json_string(nav_json_string);
    }

    pub fn write_alocs_to_nav_json_file(&mut self, nav_json_file: String) {
        println!("Writing EntitiesJson to nav.json file: {}", nav_json_file);
        io::stdout().flush().unwrap();
        let nav_json_string = serde_json::to_string(&self.alocs).unwrap();
        fs::write(nav_json_file, nav_json_string.as_str())
            .expect("Should have been able to write to the file");
    }

    pub fn build_from_nav_json_string(alocs_json_string: String) -> EntitiesJson {
        return serde_json::from_str(&alocs_json_string).expect("JSON was not well-formatted");
    }

    pub fn output_entities(&mut self) {
        for entity in &self.alocs {
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
pub struct AlocHashPair {
    pub hash: String,
    pub entity: Aloc,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PfBoxHashPair {
    pub hash: String,
    pub entity: PfBox,
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
    pub size: Scale,
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
