use std::fs;
use serde_json::Value;
use tauri::api::path::app_config_dir;
use std::sync::Once;

static START: Once = Once::new();
static mut CONFIG_URL: Option<String> = None;

pub fn get_config_url() -> String {
    unsafe {
        START.call_once(|| {
            let config_dir = app_config_dir(&tauri::Config::default())
                .expect("Failed to get config directory");
            let config_path = config_dir.join("BioMetric-SYNC_config.json");

            let config_content = fs::read_to_string(&config_path)
                .unwrap_or_else(|_| {
                    println!("Config file not found at {:?}, using default URL", config_path);
                    "{}".to_string()
                });
            let config: Value = serde_json::from_str(&config_content)
                .unwrap_or_else(|_| serde_json::json!({}));

            CONFIG_URL = Some(
                config["url"]
                    .as_str()
                    .unwrap_or("http://default-url.com")
                    .to_string()
            );
        });

        CONFIG_URL.clone().unwrap()
    }
}
