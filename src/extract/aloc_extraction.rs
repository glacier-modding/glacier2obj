use std::thread::{self, ScopedJoinHandle};
use std::{collections::{HashMap, HashSet}, fs, io::{self, Write}, path::{Path, PathBuf}};

use itertools::Itertools;
use rpkg_rs::misc::hash_path_list::PathList;
use rpkg_rs::resource::{partition_manager::PartitionManager, resource_package::ResourcePackage, runtime_resource_id::RuntimeResourceID};

use crate::{json_serde::entities_json::EntitiesJson, package::package_scan::PackageScan};

pub struct AlocExtraction;

impl AlocExtraction {
    pub fn extract_alocs(runtime_folder: String, needed_aloc_hashes: HashSet<String>, partition_manager: &PartitionManager, alocs_output_folder: String) {
        let aloc_count = needed_aloc_hashes.len();
        let needed_aloc_hashes_list = Vec::from_iter(needed_aloc_hashes);
        let target_num_threads = 10;
        let alocs_output_folder_ref = &alocs_output_folder;
        let runtime_folder_ref = &runtime_folder;
        println!("Creating directory '{}' if it doesn't exist.", alocs_output_folder);
        fs::create_dir_all(alocs_output_folder_ref).expect("Failed to create aloc folder");
        println!("Found {} needed alocs. Using {} threads of ~{} alocs per thread.", aloc_count, target_num_threads, aloc_count / target_num_threads);
        thread::scope(|scope| {
            let mut handles: Vec<ScopedJoinHandle<()>> = Vec::new();
            for (thread_num, chunk) in needed_aloc_hashes_list.chunks(aloc_count.div_ceil(target_num_threads)).enumerate() {
                handles.push(scope.spawn(move || {
                    let alocs_output_folder_path = PathBuf::from(String::from(alocs_output_folder_ref));
                    let mut resource_packages: HashMap<String, ResourcePackage> = HashMap::new();

                    let mut i = 0;
                    for hash in chunk {
                        i += 1;
                        let runtime_folder_path = PathBuf::from(runtime_folder_ref);
                        let rrid: RuntimeResourceID = RuntimeResourceID::from_hex_string(hash.as_str()).expect(format!("Error getting RRID from hash: {}", hash.as_str()).as_str());
                        let resource_info = PackageScan::get_resource_info(partition_manager, &rrid).unwrap();
                        let last_partition = resource_info.last_partition;
                        let package_path = runtime_folder_path.join(last_partition.clone());
                        let rpkg = resource_packages.entry(last_partition.clone()).or_insert(ResourcePackage::from_file(&package_path).unwrap_or_else(|e| {
                            println!("Failed parse resource package: {}", e);
                            io::stdout().flush().unwrap();
                            std::process::exit(0)
                        }));
                        let aloc_contents = rpkg
                            .read_resource(&package_path, &rrid)
                            .unwrap_or_else(|e| {
                                println!("Failed extract resource: {}", e);
                                io::stdout().flush().unwrap();
                                std::process::exit(0)
                            });

                        let aloc_file_path_buf = alocs_output_folder_path.join(hash.clone() + ".ALOC");
                        let aloc_file_path = aloc_file_path_buf.as_os_str().to_str().unwrap();
                        println!("Thread: {}: {} / {} Extracting {} from {} and saving to '{}'", thread_num, i, chunk.len(), hash, last_partition, aloc_file_path);
                        io::stdout().flush().unwrap();
                        fs::write(aloc_file_path, aloc_contents).expect("File failed to be written");
                    }     
                }));
            }

            for thread_handle in handles {
                thread_handle.join().unwrap();
            }
        });

    }

    pub fn get_all_aloc_hashes(nav_json: &EntitiesJson, alocs_output_folder: String) -> HashSet<String> {
        let mut aloc_hashes: HashSet<String> = HashSet::new();
        let mut needed_hashes: HashSet<String> = HashSet::new();
        for entity in &nav_json.alocs {
            aloc_hashes.insert(entity.hash.clone());
        }
        let alocs_output_folder_path = PathBuf::from(&alocs_output_folder);
        for hash in aloc_hashes {
            let aloc_file_path_buf = alocs_output_folder_path.join(hash.clone() + ".ALOC");
            let aloc_file_path = aloc_file_path_buf.as_os_str().to_str().unwrap();
            if Path::new(aloc_file_path).exists() {
                println!("{} already exists, skipping extraction.", aloc_file_path);
                io::stdout().flush().unwrap();
                continue;
            }
            needed_hashes.insert(hash);
        }
        return needed_hashes;
    }
}
