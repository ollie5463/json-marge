extern crate json_value_merge;

use std::fmt::format;
use std::fs;
use std::fs::{DirEntry, ReadDir};
use std::io::Read;
use std::path::PathBuf;
use serde_json::Value;
use json_value_merge::Merge;

fn main() {
    let config_version = "1.1.0";
    let venture_name = "ballybetnj";
    let environment = "development";
    let path_to_configs = "../microsoft-azure-config";

    let path_to_write_to = "./test";

    let mappings = create_file_mappings(venture_name.to_string(), config_version.to_string(), environment.to_string());
    for mapping in mappings {
        let base_file = fs::read_to_string(&mapping.0).expect("");
        let override_file = fs::read_to_string(mapping.1).expect("");
        let mut base_file_json: Value = serde_json::from_str(base_file.as_str()).unwrap();
        let override_file_json: Value = serde_json::from_str(override_file.as_str()).unwrap();
        base_file_json.merge(&override_file_json);
        println!("{:?}", base_file_json);
        let file_name = mapping.0.file_name().unwrap().to_str().unwrap();
        let parent_file_path = format!("{}/{}/{}/{}", path_to_write_to, venture_name, config_version, environment);
        let file_path = std::path::Path::new(&parent_file_path);
        fs::create_dir_all(file_path).expect("TODO: panic message");
        let full_path = format!("{}/{}", parent_file_path, file_name);
        fs::write(full_path, base_file_json.to_string()).expect("TODO: panic message");
    }
}

fn create_file_mappings(venture_name: String, config_version: String, environment: String) -> Vec<(PathBuf, PathBuf)> {
    let mut file_mappings: Vec<(PathBuf, PathBuf)> = vec!();
    for base_path in fs::read_dir(format!("../microsoft-azure-config/{}/{}/base", venture_name, config_version)).unwrap() {
        if let Ok(ok_base_path) = base_path {
            for override_path in fs::read_dir(format!("../microsoft-azure-config/{}/{}/{}", venture_name, config_version, environment)).unwrap() {
                if let Ok(ok_override_path) = override_path {
                    if ok_override_path.file_name() == ok_base_path.file_name() {
                        println!("pushing {:?}", ok_base_path.path());
                        println!("pushing {:?}", ok_override_path.path());
                        file_mappings.push((ok_base_path.path(), ok_override_path.path()));
                    }
                }
            };
        };
    };
    file_mappings
}