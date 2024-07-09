use std::fs;
use serde_json::Value;
use tauri::api::path::app_config_dir;
use std::sync::Once;

static CONFIG_URL_ONCE: Once = Once::new();
static DB_URL_ONCE: Once = Once::new();
static DB_NAME_ONCE: Once = Once::new();

static mut CONFIG_URL: Option<String> = None;
static mut DB_URL: Option<String> = None;
static mut DB_NAME: Option<String> = None;

fn initialize_config() {
    let config_dir = app_config_dir(&tauri::Config::default())
        .expect("Failed to get config directory");
    let config_path = config_dir.join("BioMetric-SYNC_config.json");

    let config_content = fs::read_to_string(&config_path)
        .unwrap_or_else(|_| {
            println!("Config file not found at {:?}, using default values", config_path);
            "{}".to_string()
        });

    let config: Value = serde_json::from_str(&config_content)
        .unwrap_or_else(|_| {
            println!("Failed to parse config, using empty object");
            serde_json::json!({})
        });

    unsafe {
        CONFIG_URL = Some(
            config["GATEWAY_URL"]
                .as_str()
                .unwrap_or("http://default_gateway-url.com")
                .to_string()
        );

        DB_URL = Some(
            config["DB_URL"]
                .as_str()
                .unwrap_or("mongodb://localhost:27017")
                .to_string()
        );

        DB_NAME = Some(
            config["DB_NAME"]
                .as_str()
                .unwrap_or("hrm")
                .to_string()
        );
    }

    println!("Config values initialized: CONFIG_URL: {:?}, DB_URL: {:?}, DB_NAME: {:?}", unsafe { CONFIG_URL.clone() }, unsafe { DB_URL.clone() }, unsafe { DB_NAME.clone() });
}

pub fn get_config_url() -> String {
    unsafe {
        CONFIG_URL_ONCE.call_once(|| {
            initialize_config();
        });

        CONFIG_URL.clone().unwrap()
    }
}

pub fn get_db_url() -> String {
    println!("Inside get_db_url function");
    unsafe {
        DB_URL_ONCE.call_once(|| {
            initialize_config();
        });

        DB_URL.clone().unwrap_or_else(|| {
            println!("DB_URL is None!");
            "mongodb://localhost:27017".to_string() // Provide a default value if DB_URL is None
        })
    }
}

pub fn get_db_name() -> String {
    unsafe {
        DB_NAME_ONCE.call_once(|| {
            initialize_config();
        });

        DB_NAME.clone().unwrap()
    }
}
