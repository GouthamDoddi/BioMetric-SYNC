use serde::{Deserialize, Serialize};
use reqwest::{header::{HeaderMap, HeaderValue, CONTENT_TYPE}, Client, Response};
use serde_json::json;
use tauri::command;
use diqwest::WithDigestAuth;
use crate::config::get_config_url;


#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResult {
    pub MatchList: Vec<Match>,
    pub numOfMatches: u32,
    pub totalMatches: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Match {
    pub Device: Device,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Device {
    pub ISAPIParams: ISAPIParams,
    pub activeStatus: bool,
    pub devIndex: String,
    pub devMode: String,
    pub devName: String,
    pub devStatus: String,
    pub devType: String,
    pub devVersion: String,
    pub protocolType: String,
    pub videoChannelNum: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ISAPIParams {
    pub address: String,
    pub addressingFormatType: String,
    pub portNo: u16,
}

#[command]
pub async fn get_all_devices(company_key: String) -> Result<String, String> {
    // let url = get_config_url().unwrap_or_else(|| "http://192.168.1.186:586".to_string());
    let url = get_config_url();
    println!("Using URL: {}", url);
    let api_url = format!("{url}/ISAPI/ContentMgmt/DeviceMgmt/deviceList?format=json");
    let username = "admin";
    let password = "Admin@123";

    // Set headers
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    // Create the JSON body
    let data = json!({
        "SearchDescription": {
            "position": 0,
            "maxResult": 100,
            "Filter": {
                "key": company_key,
                "devType": "AccessControl",
                "protocolType": ["ISAPI"],
                "devStatus": ["online", "offline"]
            }
        }
    });

    // Send the POST request with Digest authentication
    let response = Client::new()
        .post(api_url)
        .headers(headers)
        .json(&data)
        .send_with_digest_auth(username, password)
        .await
        .map_err(|e| e.to_string())?;

    // Print response status and body for debugging
    let status = response.status();
    let body = response.text().await.map_err(|e| e.to_string())?;
    // println!("Status: {}", status);
    // println!("Body: {}", body);

    if status.is_success() {
        // let search_result: SearchResult = serde_json::from_str(&body).map_err(|e| e.to_string())?;
        Ok(body)
    } else {
        Err(format!("Failed to fetch devices: {}", status))
    }
}