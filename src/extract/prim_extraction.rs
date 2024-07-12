use std::{collections::{HashMap, HashSet}, fs, io::{self, Write}, path::{Path, PathBuf}};

use rpkg_rs::resource::{partition_manager::PartitionManager, resource_package::ResourcePackage, runtime_resource_id::RuntimeResourceID};

use crate::{json_serde::prims_json::PrimsJson, package::package_scan::PackageScan};

pub struct PrimExtraction;

impl PrimExtraction {
    pub fn extract_prims(runtime_folder: String, needed_prim_hashes: HashSet<String>, partition_manager: &PartitionManager, prims_output_folder: String) {
        let runtime_folder_path = PathBuf::from(&runtime_folder);
        let prims_output_folder_path = PathBuf::from(&prims_output_folder);
        let mut resource_packages: HashMap<String, ResourcePackage> = HashMap::new();
        let prim_count = needed_prim_hashes.len();
        let mut i = 0;
        for hash in needed_prim_hashes {
            i += 1;
            let rrid: RuntimeResourceID = RuntimeResourceID::from_hex_string(hash.as_str()).unwrap();
            let resource_info = PackageScan::get_resource_info(partition_manager, &rrid).unwrap();
            let last_partition = resource_info.last_partition;
            let package_path = runtime_folder_path.join(last_partition.clone());
            let rpkg = resource_packages.entry(last_partition.clone()).or_insert(ResourcePackage::from_file(&package_path).unwrap_or_else(|e| {
                println!("Failed parse resource package: {}", e);
                io::stdout().flush().unwrap();
                std::process::exit(0)
            }));
            let prim_contents = rpkg
                .read_resource(&package_path, &rrid)
                .unwrap_or_else(|e| {
                    println!("Failed extract resource: {}", e);
                    io::stdout().flush().unwrap();
                    std::process::exit(0)
                });

            let prim_file_path_buf = prims_output_folder_path.join(hash.clone() + ".PRIM");
            let prim_file_path = prim_file_path_buf.as_os_str().to_str().unwrap();
            println!("{} / {} Extracting {} from {} and saving to '{}'", i, prim_count, hash, last_partition, prim_file_path);
            io::stdout().flush().unwrap();
            fs::write(prim_file_path, prim_contents).expect("File failed to be written");
        }

    }

    pub fn get_needed_prim_hashes(prims_json: &PrimsJson, prims_output_folder: String) -> HashSet<String> {
        let mut hashes: HashSet<String> = HashSet::new();
        let mut needed_hashes: HashSet<String> = HashSet::new();
        for entity in &prims_json.entities {
            hashes.insert(entity.prim_hash.clone());
        }
        let prims_output_folder_path = PathBuf::from(&prims_output_folder);
        for hash in hashes {
            let prim_file_path_buf = prims_output_folder_path.join(hash.clone() + ".PRIM");
            let prim_file_path = prim_file_path_buf.as_os_str().to_str().unwrap();
            if Path::new(prim_file_path).exists() {
                println!("{} already exists, skipping extraction.", prim_file_path);
                io::stdout().flush().unwrap();
                continue;
            }
            needed_hashes.insert(hash);
        }
        return needed_hashes;
    }
}
