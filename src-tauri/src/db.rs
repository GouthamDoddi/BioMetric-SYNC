use mongodb::{Client, Collection, options::ClientOptions, bson::Document};
use std::error::Error;
use crate::config::{ get_db_url, get_db_name };

pub async fn get_mongodb_collection() -> Result<Collection<Document>, bool> {
    match try_get_mongodb_collection().await {
        Ok(collection) => Ok(collection),
        Err(e) => {
            eprintln!("Failed to get MongoDB collection: {}", e);
            eprintln!("Error details: {:?}", e);
            // Err(Box::newError("db conect failed")))
            Err(false)
        }
    }
}

async fn try_get_mongodb_collection() -> Result<Collection<Document>, Box<dyn Error>> {
    let db_uri = get_db_url();

    println!("{} is the db url", db_uri);

    // Parse the MongoDB connection URI
    let client_options = ClientOptions::parse(db_uri.as_str()).await?;

    // Connect to the MongoDB server
    let client = Client::with_options(client_options)?;

    // Access a specific database
    let db = client.database("hrm");

    let collection = db.collection::<Document>("BioMetricActivity");

    Ok(collection)
}

pub async fn get_mongodb_user_data_collection() -> Result<Collection<Document>, Box<dyn Error>> {
    // MongoDB connection URI
    let db_uri = get_db_url();

    // Parse the MongoDB connection URI
    let client_options = ClientOptions::parse(db_uri.as_str()).await?;

    // Optionally configure client options (e.g., SSL settings, connection pool options)

    // Connect to the MongoDB server
    let client = Client::with_options(client_options)?;

    // Access a specific database
    let db = client.database(&get_db_name());

    // Use `db` to access collections and perform operations

    // println!("Successfully connected to MongoDB!");


    let collection = db.collection::<Document>("BioMetricUserData");

    Ok(collection)
}


