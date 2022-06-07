use bundlebee::BundleBee;
use serde_json::{self, json};
use std;

#[test]
fn create_config_file_test() {
    let config_file_path = "./tests/test_files/create_config_file_test/bundlebee_config.json";
    let b = BundleBee::new();
    b.create_config_file(config_file_path);
    assert!(std::path::Path::new(config_file_path).exists());
    let config_file_content = match std::fs::read_to_string(config_file_path) {
        Ok(content) => content,
        Err(e) => panic!("{}", e),
    };
    let config: serde_json::Value = match serde_json::from_str(&config_file_content) {
        Ok(content) => content,
        Err(e) => {
            println!("{}", e);
            json!({})
        },
    };
    assert_eq!(config["imports"], json!([]));
    assert_eq!(config["exports"], json!([]));
    
    //match std::fs::remove_file(config_file_path) {
     //   Ok(_) => (),
     //   Err(e) => panic!("{}", e),
    //};
}