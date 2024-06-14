use itertools::Itertools;
use rpkg_rs::misc::hash_path_list::PathList;
use rpkg_rs::misc::ini_file_system::IniFileSystem;
use rpkg_rs::misc::resource_id::ResourceID;
use rpkg_rs::resource::partition_manager::{PartitionManager, PartitionState};
use rpkg_rs::resource::pdefs::PackageDefinitionSource;
use rpkg_rs::resource::resource_info::ResourceInfo;
use rpkg_rs::resource::resource_partition::PatchId;
use rpkg_rs::resource::runtime_resource_id::RuntimeResourceID;
use std::collections::{HashSet, VecDeque};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::{fs, io};

pub struct ScenarioScan {
    retail_folder: String,
    game_version: String,
    scenario: String,
    hash_list_file: String,
    hashes_for_output: HashSet<RuntimeResourceID>,
    alocs_for_output: HashSet<RuntimeResourceID>,
    prims_for_output: HashSet<RuntimeResourceID>,
    
}

impl ScenarioScan {
    pub fn new(retail_folder: String, game_version: String, scenario: String, hash_list_file: String) -> Self {
        Self {
            retail_folder,
            game_version,
            scenario,
            hash_list_file,
            hashes_for_output: HashSet::new(),
            alocs_for_output: HashSet::new(),
            prims_for_output: HashSet::new(),
        }
    }

    pub fn scan_scenario(&mut self) {
        let retail_path = PathBuf::from(&self.retail_folder);
        let thumbs_path = retail_path.join("thumbs.dat");

        let thumbs = IniFileSystem::from(&thumbs_path.as_path()).unwrap_or_else(|err| {
            eprintln!("Error reading thumbs file: {:?}", err);
            std::process::exit(1);
        });

        let app_options = &thumbs.root()["application"];

        let hash_list_path = Path::new(&self.hash_list_file);

        let mut path_list = PathList::new();
        path_list.parse_into(hash_list_path).unwrap();
            
        
        if let (Some(proj_path), Some(relative_runtime_path)) = (
            app_options.options().get("PROJECT_PATH"),
            app_options.options().get("RUNTIME_PATH"),
        ) {
            let runtime_path = PathBuf::from(format!(
                "{}\\{proj_path}\\{relative_runtime_path}",
                retail_path.display()
            ));
            std::println!("start reading package definitions {:?}", runtime_path);

            let mut package_manager = PartitionManager::new(runtime_path.clone());

            //read the packagedefs here
            let mut last_index = 0;
            let mut progress = 0.0;
            let progress_callback = |current, state: &PartitionState| {
                if current != last_index {
                    last_index = current;
                    print!("Mounting partition {} ", current);
                }
                let install_progress = (state.install_progress * 10.0).ceil() / 10.0;

                let chars_to_add = (install_progress * 10.0 - progress * 10.0) as usize * 2;
                let chars_to_add = std::cmp::min(chars_to_add, 20);
                print!("{}", "█".repeat(chars_to_add));
                io::stdout().flush().unwrap();

                progress = install_progress;

                if progress == 1.0 {
                    progress = 0.0;
                    println!(" done :)");
                }
            };

            let package_defs_bytes =
                std::fs::read(runtime_path.join("packagedefinition.txt").as_path()).unwrap();

            let mut package_defs = match self.game_version.as_str() {
                "HM2016" => PackageDefinitionSource::HM2016(package_defs_bytes).read(),
                "HM2" => PackageDefinitionSource::HM2(package_defs_bytes).read(),
                "HM3" => PackageDefinitionSource::HM3(package_defs_bytes).read(),
                e => {
                    eprintln!("invalid game version: {}", e);
                    std::process::exit(0);
                }
            }
            .unwrap_or_else(|e| {
                println!("Failed to parse package definitions {}", e);
                std::process::exit(0);
            });

            for partition in package_defs.iter_mut() {
                partition.set_max_patch_level(301);
            }

            package_manager
                .mount_partitions(
                    PackageDefinitionSource::Custom(package_defs),
                    progress_callback,
                )
                .unwrap_or_else(|e| {
                    eprintln!("failed to init package manager: {}", e);
                    std::process::exit(0);
                });

            let ioi_string_or_hash = self.scenario.as_str();
            let mut hash;
            let hash_resource_id = RuntimeResourceID::from_hex_string(ioi_string_or_hash);
            if hash_resource_id.is_err() {
                let ioi_string_resource_id = ResourceID::from_str(ioi_string_or_hash);
                if !ioi_string_resource_id.is_err() {
                    hash = RuntimeResourceID::from_resource_id(&ioi_string_resource_id.unwrap()).to_hex_string();
                } else {
                    println!("Invalid RuntimeResourceId");
                    std::process::exit(0);
                }
            } else {
                hash = ioi_string_or_hash.to_string();
            }
            let mut hashes: VecDeque<String> = VecDeque::from([String::from_str(&hash).unwrap()]);
            let mut found_hashes = HashSet::new();
            println!("Getting ALOCs for: {}", hash);
            loop {
                if hashes.len() == 0 {
                    break;
                }
                hash = hashes.pop_front().unwrap();
                let rrid = RuntimeResourceID::from_hex_string(&hash).unwrap_or_else(|_| {
                    println!("Invalid RuntimeResourceId");
                    std::process::exit(0);
                });
                if found_hashes.contains(&rrid) {
                    continue;
                }
                found_hashes.insert(rrid);
                let resource_package_opt = ScenarioScan::get_resource_info(&package_manager, &rrid);
                if resource_package_opt.is_none() {
                    continue;
                }
                let ioi_string = if path_list.get(&rrid).is_some() {
                    path_list.get(&rrid).unwrap().resource_path()
                } else {
                    "".to_string()
                };
        
                let resource_package = resource_package_opt.unwrap();
                let references = resource_package.0.references();
                let mut prim_rrid: Option<RuntimeResourceID> = None;
                let mut prim_ioi_string: Option<String> = None;
                let mut prim_partition: Option<String> = None;
                let mut has_aloc = false;
                for reference in references.iter() {
                    let dep_rrid = reference.0;
                    
                    if found_hashes.contains(&dep_rrid) {
                        continue;
                    }
                    let depend_resource_opt = ScenarioScan::get_resource_info(&package_manager, &dep_rrid);
                    if depend_resource_opt.is_none() {
                        continue;
                    }
                    let dep_ioi_string = if path_list.get(&dep_rrid).is_some() {
                        path_list.get(&dep_rrid).unwrap().resource_path()
                    } else {
                        "".to_string()
                    };

                    let depend_resource = depend_resource_opt.unwrap();
                    
                    if depend_resource.0.data_type().as_str() == "PRIM" {
                        prim_rrid = Some(dep_rrid);
                        prim_ioi_string = Some(dep_ioi_string.clone());
                        prim_partition = Some(depend_resource.1.clone());
                    }
                    if Vec::from(["TEMP", "ALOC", "PRIM"]).contains(&depend_resource.0.data_type().as_str()) {
                        let is_aloc = depend_resource.0.data_type().as_str() == "ALOC";
                        if is_aloc {
                            println!("{} {} Type: {} Partition: {}", rrid, ioi_string, resource_package.0.data_type(), resource_package.1);
                            println!("|-> {} {} Type: {} Partition: {}", dep_rrid, dep_ioi_string, depend_resource.0.data_type(), depend_resource.1);
                            self.hashes_for_output.insert(rrid);
                            self.alocs_for_output.insert(dep_rrid);
                            has_aloc = true;
                        }
                        hashes.push_back(dep_rrid.to_hex_string());
                    }
                }
                if has_aloc && prim_rrid.is_some() {
                    self.prims_for_output.insert(prim_rrid.unwrap());
                    println!("|-> {} {} Type: prim Partition: {}", prim_rrid.unwrap(), prim_ioi_string.unwrap(), prim_partition.unwrap());

                }
            }
        } else {
            eprintln!(
                "Missing required properties inside thumbs.dat:\n\
                PROJECT_PATH: {}\n\
                RUNTIME_PATH: {}",
                app_options.has_option("PROJECT_PATH"),
                app_options.has_option("RUNTIME_PATH")
            );
            return;
        }
    }

    pub fn output_to_file(&self, output_file: String) {
        let output_path = Path::new(&output_file);
        let mut data = String::from(r#"{"type":"listPrimEntities", "prims":["#);
        let mut it = self.prims_for_output.iter().peekable();
        while let Some(rrid) = it.next() {
            data += (String::from("\"") + rrid.to_hex_string().as_str() + "\"").as_str();
            if it.peek().is_some() {
                data += String::from(",").as_str();
            }
        }
        data += String::from("]}").as_str();
        fs::write(output_path, data).expect("Unable to write file");
    }

    fn get_resource_info(package_manager: &PartitionManager, rrid: &RuntimeResourceID) -> Option<(ResourceInfo, String)>  {
        let mut last_occurrence: Option<&ResourceInfo> = None;
        let mut last_partition: Option<String> = None;
        for partition in package_manager.partitions() {
            let changes = partition.resource_patch_indices(rrid);
            let deletions = partition.resource_removal_indices(rrid);
            let occurrences = changes
                .clone()
                .into_iter()
                .chain(deletions.clone().into_iter())
                .collect::<Vec<PatchId>>();
            for occurrence in occurrences.iter().sorted() {
                if deletions.contains(occurrence) {
                    last_occurrence = None;
                }
                if changes.contains(occurrence) {
                    if let Ok(info) = partition.resource_info_from(rrid, *occurrence) {
                        last_occurrence = Some(info);
                        last_partition = Some(partition.partition_info().filename(*occurrence));
                    }
                }
            }
            if !last_occurrence.is_none(){
                break;
            }
        }
        if last_occurrence.is_none() || last_partition.is_none() {
            return None
        }
        return Some((last_occurrence.unwrap().clone(), last_partition.unwrap()));
    }
}