use std;
use serde_json::{self, json};

pub struct BundleBee {}

impl BundleBee {
    pub fn new() -> Self {
        Self {  }
    }

    pub fn create_config_file(&self, config_file_path: &str) {
        let path = std::path::Path::new(config_file_path);

        let prefix = match path.parent() {
            Some(p) => p.to_str().unwrap().to_string(),
            None => "./".to_string(),
        };

        match std::fs::create_dir_all(prefix) {
            Ok(_) => {},
            Err(e) => {
                println!("{}", e);
            },
        }

        let config = json!({
            "imports" : [],
            "exports" : []
        });
        
        match std::fs::write(config_file_path, config.to_string()){
            Ok(_) => {},
            Err(e) => {
                println!("{}", e);
            },
        }
    }
}