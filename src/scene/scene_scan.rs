use rpkg_rs::misc::hash_path_list::PathList;
use rpkg_rs::misc::resource_id::ResourceID;
use rpkg_rs::resource::partition_manager::PartitionManager;
use rpkg_rs::resource::runtime_resource_id::RuntimeResourceID;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::Write;
use std::path::Path;
use std::str::FromStr;
use std::{fs, io};

use crate::package::package_scan::PackageScan;

pub struct SceneScan {
    bricks: Vec<String>,
    hash_list_file: String,
    hashes_for_output: HashSet<RuntimeResourceID>,
    alocs_for_output: HashSet<RuntimeResourceID>,
    prims_for_output: HashSet<RuntimeResourceID>,
    
}

impl SceneScan {
    pub fn new(bricks: Vec<String>, hash_list_file: String) -> Self {
        Self {
            bricks,
            hash_list_file,
            hashes_for_output: HashSet::new(),
            alocs_for_output: HashSet::new(),
            prims_for_output: HashSet::new(),
        }
    }

    // This function was provided by Atampy26
    fn get_brick_temp_hashes(tblu_hashes: Vec<String>, partition_manager: &PartitionManager) -> Vec<String> {
        println!("Getting scene temps from scene tblus.");
        // Ensure we only get the references from the lowest chunk version of each resource (matches the rest of GK's behaviour)
        let mut reverse_dependencies: HashMap<String, Vec<String>> = HashMap::new();
		let resources = partition_manager
        .partitions()
        .into_iter()
        .rev()
        .flat_map(|partition| {
            partition
                .latest_resources()
                .into_iter()
                .map(|(resource, _)| (resource.rrid(), resource.references()))
        })
        .collect::<HashMap<_, _>>();

    reverse_dependencies
        .try_reserve(resources.len())
        .expect("Failed to get reverse dependencies.");

    reverse_dependencies.extend(resources.keys().map(|x| (x.to_hex_string(), Default::default())));

    resources
        .into_iter()
        .flat_map(|(resource_id, resource_references)| {
            let res_id_str = resource_id.to_hex_string();

            resource_references
                .iter()
                .map(move |(reference_id, _)| (reference_id.to_hex_string(), res_id_str.to_owned()))
        })
        .for_each(|(key, value)| {
            if let Some(x) = reverse_dependencies.get_mut(&key) {
                x.push(value);
            }
        });
        let mut brick_temp_hashes: Vec<String> = Vec::new();
        for tblu_hash in tblu_hashes {
            brick_temp_hashes.push(reverse_dependencies.get(&tblu_hash).unwrap().first().unwrap().to_string());
        }
        return brick_temp_hashes;
    }

    pub fn scan_scenario(&mut self, partition_manager: &PartitionManager) {
        let hash_list_path = Path::new(&self.hash_list_file);

        let mut path_list = PathList::new();
        path_list.parse_into(hash_list_path).unwrap();
        let brick_temp_hashes = SceneScan::get_brick_temp_hashes(self.bricks.clone(), partition_manager);
        for brick_temp_hash in brick_temp_hashes {
            let mut hash = brick_temp_hash;
            let mut hashes: VecDeque<String> = VecDeque::from([String::from_str(&hash).unwrap()]);
            let mut found_hashes = HashSet::new();
            println!("Getting ALOCs for: {}", hash);
            io::stdout().flush().unwrap();


            // These hashes are for things that aren't needed for navmeshes like fx ghost mode, fx torus, and shockwave sphere
            // 00BDA629523CE8B2 [assembly:/_pro/effects/templates/misc/fx_ghostmode.template?/fx_e_ghostmode_outfit_manipulator.entitytemplate].pc_entitytype
            // 00ACD408BE462DD3 fx_shockwave_sphere_1m (template)
            // 00355E794876922A [assembly:/_pro/effects/geometry/misc/fx_basic_shapes.wl2?/fx_torus_1m.prim].pc_entitytype
            // 00B4ED8EA7D2F405 [assembly:/_pro/effects/geometry/gameplay/fx_gameplay_invisibleshotcoli.wl2?/invisibleshotcoli_box20cm.prim].pc_prim
            // 0069A9533284DCE8 [assembly:/_pro/effects/geometry/misc/fx_basic_shapes.wl2?/fx_torus_1m.prim].pc_prim
            // 009E5756C710494E [assembly:/_pro/effects/geometry/glow/fx_glow_generic_planes.wl2?/fx_glow_generic_plane_j_00.prim].pc_prim
            // 00D347CBA29EE6BA [assembly:/_pro/characters/templates/hero/agent47/agent47.template?/agent47_default.entitytemplate].pc_entitytype
            // 00E74E523354AA2F [assembly:/_pro/environment/geometry/foliage/palm_queen_a.wl2?/palm_queen_15m_c.prim].pc_prim
            // 0081EC97AC8CA5BB [assembly:/templates/aspectdummy.aspect]([modules:/zgeomentity.class].entitytype,[modules:/zdynamicphysicsaspect.class].entitytype,[modules:/zcollisionresourceshapeaspect.class].entitytype).pc_entitytype
            let excluded_hashes: Vec<&str> = Vec::from(["00BDA629523CE8B2", "00ACD408BE462DD3","00355E794876922A","00B4ED8EA7D2F405","0069A9533284DCE8","009E5756C710494E","00D347CBA29EE6BA","00E74E523354AA2F", "0081EC97AC8CA5BB"]);
            loop {
                if hashes.len() == 0 {
                    break;
                }
                hash = hashes.pop_front().unwrap();
                
                if excluded_hashes.contains(&hash.as_str()) {
                    continue
                }
                let rrid = RuntimeResourceID::from_hex_string(&hash).unwrap_or_else(|_| {
                    println!("Invalid RuntimeResourceId");
                    io::stdout().flush().unwrap();

                    std::process::exit(0);
                });
                if found_hashes.contains(&rrid) {
                    continue;
                }
                found_hashes.insert(rrid);
                let resource_package_opt = PackageScan::get_resource_info(partition_manager, &rrid);
                if resource_package_opt.is_none() {
                    continue;
                }
        
                let resource_package = resource_package_opt.unwrap();
                let references = resource_package.last_occurrence.references();
                let mut prim_rrid: Option<RuntimeResourceID> = None;
                let mut prim_ioi_string: Option<String> = None;
                let mut prim_partition: Option<String> = None;
                let mut has_aloc = false;
                let mut should_include = true;
                for reference in references.iter() {
                    let dep_rrid = reference.0;
                    
                    if excluded_hashes.contains(&dep_rrid.to_hex_string().as_str()) {
                        should_include = false;
                        continue
                    }
                    let depend_resource_opt = PackageScan::get_resource_info(partition_manager, &dep_rrid);
                    if depend_resource_opt.is_none() {
                        continue;
                    }
                    let dep_ioi_string = if path_list.get(&dep_rrid).is_some() {
                        path_list.get(&dep_rrid).unwrap().resource_path()
                    } else {
                        "".to_string()
                    };

                    let depend_resource = depend_resource_opt.unwrap();
                    if depend_resource.last_occurrence.data_type().as_str() == "PRIM" {
                        prim_rrid = Some(dep_rrid);
                        prim_ioi_string = Some(dep_ioi_string.clone());
                        prim_partition = Some(depend_resource.last_partition.clone());
                    }
                    if Vec::from(["TEMP", "ALOC", "PRIM", "ASET"]).contains(&depend_resource.last_occurrence.data_type().as_str()) {
                        let is_aloc = depend_resource.last_occurrence.data_type().as_str() == "ALOC";
                        if is_aloc {
                            self.hashes_for_output.insert(rrid);
                            self.alocs_for_output.insert(dep_rrid);
                            has_aloc = true;
                        }
                        hashes.push_back(dep_rrid.to_hex_string());
                    }
                }
                if should_include && has_aloc && prim_rrid.is_some() {
                    self.prims_for_output.insert(prim_rrid.unwrap());
                    println!("Found PRIM: {} {} in {}", prim_rrid.unwrap(), prim_ioi_string.unwrap(), prim_partition.unwrap());
                    io::stdout().flush().unwrap();

                }
            }
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
}