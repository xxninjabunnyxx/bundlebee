use serde_json::json;
pub struct BundleBee {}

impl BundleBee {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create_config_file(&self, config_file_path: &str) {
        let config_file_content = json!({
            "imports": [],
            "exports": [],
        });
        let config_file_content_str = match serde_json::to_string(&config_file_content) {
            Ok(content) => content,
            Err(e) => panic!("{}", e),
        };
        match std::fs::write(config_file_path, config_file_content_str) {
            Ok(_) => (),
            Err(e) => panic!("{}", e),
        };
    }
}