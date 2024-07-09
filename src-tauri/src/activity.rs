use reqwest::{header::{HeaderMap, HeaderValue, CONTENT_TYPE}, Client, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tauri::command;
use diqwest::WithDigestAuth;
use rand::Rng;
use chrono::{Utc};
use mongodb::{bson::{doc, Document}, Collection};

use crate::db;
use db::get_mongodb_collection;
use crate::config::get_config_url;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Info {
    pub InternetAccess: i32,
    pub MACAddr: String,
    pub RS485No: i32,
    pub accessChannel: i32,
    pub alarmInNo: i32,
    pub alarmOutNo: i32,
    pub attendanceStatus: String,
    pub cardNo: String,
    pub cardReaderKind: i32,
    pub cardReaderNo: i32,
    pub cardType: i32,
    pub caseSensorNo: i32,
    pub deviceNo: i32,
    pub distractControlNo: i32,
    pub doorNo: i32,
    pub employeeNoString: String,
    pub localControllerID: i32,
    pub major: i32,
    pub minor: i32,
    pub multiCardGroupNo: i32,
    pub netUser: String,
    pub remoteHostAddr: String,
    pub reportChannel: i32,
    pub serialNo: i64,
    pub statusValue: i32,
    pub swipeCardType: i32,
    pub time: String,
    pub r#type: i32,
    pub verifyNo: i32,
    pub whiteListNo: i32,
    pub deviceId: Option<String>

}

// Function to convert Info to Document for MongoDB
pub fn info_to_document(info: Info) -> Document {
    doc! {
        "InternetAccess": info.InternetAccess,
        "MACAddr": info.MACAddr,
        "RS485No": info.RS485No,
        "accessChannel": info.accessChannel,
        "alarmInNo": info.alarmInNo,
        "alarmOutNo": info.alarmOutNo,
        "attendanceStatus": info.attendanceStatus,
        "cardNo": info.cardNo,
        "cardReaderKind": info.cardReaderKind,
        "cardReaderNo": info.cardReaderNo,
        "cardType": info.cardType,
        "caseSensorNo": info.caseSensorNo,
        "deviceNo": info.deviceNo,
        "distractControlNo": info.distractControlNo,
        "doorNo": info.doorNo,
        "employeeNoString": info.employeeNoString,
        "localControllerID": info.localControllerID,
        "major": info.major,
        "minor": info.minor,
        "multiCardGroupNo": info.multiCardGroupNo,
        "netUser": info.netUser,
        "remoteHostAddr": info.remoteHostAddr,
        "reportChannel": info.reportChannel,
        "serialNo": info.serialNo,
        "statusValue": info.statusValue,
        "swipeCardType": info.swipeCardType,
        "time": info.time,
        "type": info.r#type,
        "verifyNo": info.verifyNo,
        "whiteListNo": info.whiteListNo,
    }
}

fn start_of_day(dt: chrono::DateTime<Utc>) -> chrono::DateTime<Utc> {
    dt.date().and_hms(0, 0, 0)  // Set hours, minutes, and seconds to 0
}

fn end_of_day(dt: chrono::DateTime<Utc>) -> chrono::DateTime<Utc> {
    dt.date().and_hms(23, 59, 59)  // Set hours, minutes, and seconds to end of day
}

pub async fn fetch_activity_data(device_id: &String) -> Result<String, String> {
    let url = get_config_url();

    let api_url = format!("{}/ISAPI/AccessControl/AcsEvent?format=json&devIndex={}", url, device_id);
    let username = "admin";
    let password = "Admin@123";

    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    // Generate a random searchID
    let search_id = format!("{}", rand::thread_rng().gen::<u64>());
    let start_time = start_of_day(Utc::now()).to_rfc3339();  // Convert to RFC3339 format
    let end_time = end_of_day(Utc::now()).to_rfc3339();      // Convert to RFC3339 format

    // Set other parameters
    let search_result_position = 0;
    let max_results = 300;

    let data = json!({
        "AcsEventCond": {
            "searchID": search_id,
            "searchResultPosition": search_result_position,
            "maxResults": max_results,
            "startTime": start_time,
            "endTime": end_time
        }
    });

    let client = Client::new();
    let response: Response = client
    
        .post(&api_url)
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

    Ok(body)
}


async fn parse_activity_data(body: String) -> Result<Vec<Info>, String> {
    let json_response: serde_json::Value = serde_json::from_str(&body).map_err(|e| e.to_string())?;
    let info_list: Vec<Info> = serde_json::from_value(json_response["AcsEvent"]["InfoList"].clone()).map_err(|e| e.to_string())?;
    Ok(info_list)
}

#[command]
pub async fn fetch_and_upload_data(device_id: String) -> Result<Vec<String>, String> {
    // Fetch activity data (assuming fetch_activity_data is already implemented)
    let activity_data = fetch_activity_data(&device_id).await.map_err(|e| e.to_string())?;

    // Parse the activity data to extract InfoList (assuming parse_activity_data is already implemented)
    let info_list = parse_activity_data(activity_data).await.map_err(|e| e.to_string())?;

    let mut logs = Vec::new();
    // Get the MongoDB collection
    // Get the MongoDB collection
    let collection = match get_mongodb_collection().await {
        Ok(collection) => {
            // println!("Connected to Database."); // Log that the database connection is successful
            let log = format!("**Connected to Database.**");
            logs.push(log);
            collection
        },
        Err(e) => return Err(e.to_string()), // Propagate the error if collection retrieval fails
    };
    // Check and insert info list, collecting logs
    let logs = check_and_insert_info_list(info_list, collection, &device_id).await;

    Ok(logs)
}

async fn check_and_insert_info_list(info_list: Vec<Info>, collection: Collection<Document>, device_id: &String) -> Vec<String> {
    let mut logs = Vec::new();

    for mut info in info_list {
        info.deviceId = Some(device_id.to_string());

        // Check if the record already exists in MongoDB based on some identifier (e.g., serialNo)
        let filter = doc! { "serialNo": info.serialNo };
        if let Ok(Some(_)) = collection.find_one(filter.clone()).await {
            let log = format!("Record with serialNo {} already exists in the collection.", info.serialNo);
            // println!("{}", log);
            logs.push(log);
        } else {
            // If not exists, insert it into the collection
            if let Err(e) = collection.insert_one(info_to_document(info.clone())).await {
                let log = format!("Failed to insert Record with serialNo {}: {}", info.serialNo, e);
                // println!("{}", log);
                logs.push(log);
            } else {
                let log = format!("Inserted Record with serialNo {} into the collection.", info.serialNo);
                // println!("{}", log);
                logs.push(log);
            }
        }
    }

    logs
}
