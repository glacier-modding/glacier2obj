use itertools::Itertools;
use rpkg_rs::misc::ini_file_system::IniFileSystem;
use rpkg_rs::resource::partition_manager::{PartitionManager, PartitionState};
use rpkg_rs::resource::pdefs::PackageDefinitionSource;
use rpkg_rs::resource::resource_info::ResourceInfo;
use rpkg_rs::resource::resource_partition::PatchId;
use rpkg_rs::resource::runtime_resource_id::RuntimeResourceID;
use std::io::Write;
use std::path::PathBuf;
use std::io;

pub struct ResourceInfoAndPartition {
    pub last_occurrence: ResourceInfo,
    pub last_partition: String,
}

impl ResourceInfoAndPartition {
    pub fn new(last_occurrence: ResourceInfo, last_partition: String) -> Self {
        Self {
            last_occurrence,
            last_partition
        }
    }
}

#[derive(Clone)]
pub struct PackageScan;

impl PackageScan {

    pub fn scan_packages(retail_folder: String, game_version: String) -> Option<PartitionManager> {
        let mut package_manager: PartitionManager;
        let retail_path = PathBuf::from(&retail_folder);
        let thumbs_path = retail_path.join("thumbs.dat");

        let thumbs = IniFileSystem::from(&thumbs_path.as_path()).unwrap_or_else(|err| {
            eprintln!("Error reading thumbs file: {:?}", err);
            std::process::exit(1);
        });

        let app_options = &thumbs.root()["application"];
        let runtime_path: PathBuf;
        if let (Some(proj_path), Some(relative_runtime_path)) = (
            app_options.options().get("PROJECT_PATH"),
            app_options.options().get("RUNTIME_PATH"),
        ) {
            runtime_path = PathBuf::from(format!(
                "{}\\{proj_path}\\{relative_runtime_path}",
                retail_path.display()
            ));
        } else {
            eprintln!(
                "Missing required properties inside thumbs.dat:\n\
                PROJECT_PATH: {}\n\
                RUNTIME_PATH: {}",
                app_options.has_option("PROJECT_PATH"),
                app_options.has_option("RUNTIME_PATH")
            );
            return None;
        }
        std::println!("start reading package definitions {:?}", runtime_path);

        package_manager = PartitionManager::new(runtime_path.clone());

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

        let mut package_defs = match game_version.as_str() {
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
        return Some(package_manager);
    }

    pub fn get_resource_info(package_manager: &PartitionManager, rrid: &RuntimeResourceID) -> Option<ResourceInfoAndPartition>  {
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
        return Some(ResourceInfoAndPartition::new(last_occurrence.unwrap().clone(), last_partition.unwrap()));
    }
}