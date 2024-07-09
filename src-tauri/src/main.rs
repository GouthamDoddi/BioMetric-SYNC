mod devices;
mod activity;
mod db;
mod config;
mod users;

use serde::{Deserialize, Serialize};
use reqwest;
use devices::get_all_devices;
use activity::{ fetch_activity_data, fetch_and_upload_data };
use users::fetch_and_upload_users_data;

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


#[derive(Serialize, Deserialize, Debug)]
struct FakeStoreProduct {
    id: i32,
    title: String,
    price: f64,
    category: String,
    description: String,
    image: String,
}

#[tauri::command]
async fn fetch_products() -> Result<Vec<FakeStoreProduct>, String> {
    let url = "https://fakestoreapi.com/products"; // Replace with your API endpoint

    let response = reqwest::get(url).await.map_err(|err| err.to_string())?;
    if response.status().is_success() {
        let products = response.json::<Vec<FakeStoreProduct>>().await.map_err(|err| err.to_string())?;
        Ok(products)
    } else {
        Err(format!("Failed to fetch products: {}", response.status()))
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, fetch_products, 
            get_all_devices,  fetch_and_upload_data,
            fetch_and_upload_users_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
