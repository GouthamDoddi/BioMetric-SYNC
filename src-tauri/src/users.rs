use reqwest::{header::{HeaderMap, HeaderValue, CONTENT_TYPE}, Client, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tauri::command;
use diqwest::WithDigestAuth;
use mongodb::{bson::{doc, Bson, Document}, Collection};

use crate::config::get_config_url;
use crate::db::get_mongodb_user_data_collection;


#[derive(Deserialize, Serialize, Debug, Clone)]
struct UserInfo {
    employeeNo: String,
    name: String,
    userType: String,
    closeDelayEnabled: bool,
    Valid: ValidInfo,
    password: String,
    doorRight: String,
    RightPlan: Vec<RightPlanInfo>,
    maxOpenDoorTime: i32,
    openDoorTime: i32,
    localUIRight: bool,
    userVerifyMode: String,
    deviceId: Option<String>,
}

pub fn info_to_document(info: UserInfo) -> Document {
    doc! {
        "employeeNo": info.employeeNo,
        "name": info.name,
        "userType": info.userType,
        "closeDelayEnabled": info.closeDelayEnabled,
        "Valid": info.Valid,
        "password": info.password,
        "doorRight": info.doorRight,
        "RightPlan": info.RightPlan,
        "maxOpenDoorTime": info.maxOpenDoorTime,
        "openDoorTime": info.openDoorTime,
        "localUIRight": info.localUIRight,
        "userVerifyMode": info.userVerifyMode,
        "deviceId": info.deviceId,
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ValidInfo {
    enable: bool,
    timeType: String,
    beginTime: String,
    endTime: String,
}

impl Into<Bson> for ValidInfo {
    fn into(self) -> Bson {
        let doc = doc! {
            "enable": self.enable,
            "timeType": self.timeType,
            "beginTime": self.beginTime,
            "endTime": self.endTime,
        };
        Bson::Document(doc)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct RightPlanInfo {
    doorNo: i32,
    planTemplateNo: String,
}

impl Into<Bson> for RightPlanInfo {
    fn into(self) -> Bson {
        let doc = doc! {
            "doorNo": self.doorNo,
            "planTemplateNo": self.planTemplateNo
        };
        Bson::Document(doc)
    }
}


#[command]
pub async fn fetch_users_data(device_id: &String) -> Result<String, String> {
    let url = get_config_url();

    let api_url = format!("{}/ISAPI/AccessControl/UserInfo/Search?format=json&devIndex={}", url, device_id);
    let username = "admin";
    let password = "Admin@123";

    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let search_result_position: i32 = 0;
    let max_results: i32 = 300;

    let data = json!({
        "searchId": "123",
        "searchResultPosition": search_result_position,
        "maxResults": max_results
    });

    let client: Client = Client::new();
    let response: Response = client
        .post(&api_url)
        .headers(headers)
        .json(&data)
        .send_with_digest_auth(username, password)
        .await
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        return Err(format!("Request failed with status: {}", response.status()));
    }

    let body = response.text().await.map_err(|e| e.to_string())?;

    Ok(body)
}

async fn parse_users_data(body: String) -> Result<Vec<UserInfo>, String> {
    let json_response: serde_json::Value = serde_json::from_str(&body).map_err(|e| e.to_string())?;
    let info_list: Vec<UserInfo> = serde_json::from_value(json_response["UserInfoSearch"]["UserInfo"].clone()).map_err(|e| e.to_string())?;
    Ok(info_list)
}

async fn check_and_insert_user_info_list(info_list: Vec<UserInfo>, collection: Collection<Document>, device_id: &String) -> Vec<String> {
    let mut logs = Vec::new();

    for mut info in info_list {
        info.deviceId = Some(device_id.to_string());

        // Check if the record already exists in MongoDB based on some identifier (e.g., serialNo)
        let filter = doc! { "employeeNo": &info.employeeNo, "deviceId": &info.deviceId  };
        if let Ok(Some(_)) = collection.find_one(filter.clone()).await {
            let log = format!("Record with employee name {} already exists in the collection.", info.name);
            // println!("{}", log);
            logs.push(log);
        } else {
            // If not exists, insert it into the collection
            if let Err(e) = collection.insert_one(info_to_document(info.clone())).await {
                let log = format!("Failed to insert Record with employee name {}: {}", info.name, e);
                // println!("{}", log);
                logs.push(log);
            } else {
                let log = format!("Inserted Record with employee name {} into the collection.", info.name);
                // println!("{}", log);
                logs.push(log);
            }
        }
    }

    logs
}

#[command]
pub async fn fetch_and_upload_users_data(device_id: String) -> Result<Vec<String>, String> {
    // Fetch activity data (assuming fetch_activity_data is already implemented)
    let user_data = fetch_users_data(&device_id).await.map_err(|e| e.to_string())?;

    // Parse the activity data to extract InfoList (assuming parse_activity_data is already implemented)
    let info_list = parse_users_data(user_data).await.map_err(|e| e.to_string())?;

    let mut logs = Vec::new();
    // Get the MongoDB collection
    // Get the MongoDB collection
    let collection = match get_mongodb_user_data_collection().await {
        Ok(collection) => {
            // println!("Connected to Database."); // Log that the database connection is successful
            let log = format!("**Connected to Database.**");
            logs.push(log);
            collection
        },
        Err(e) => return Err(e.to_string()), // Propagate the error if collection retrieval fails
    };

    // Check and insert info list, collecting logs
    let logs = check_and_insert_user_info_list(info_list, collection, &device_id).await;

    Ok(logs)
}

