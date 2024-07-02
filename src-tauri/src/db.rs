use mongodb::{Client, Collection, options::ClientOptions, bson::Document};
use std::error::Error;

pub async fn get_mongodb_collection() -> Result<Collection<Document>, Box<dyn Error>> {
    // MongoDB connection URI
    let db_uri = format!(
        "mongodb+srv://pdtuae-hrm:{}@cluster0.mf10u.mongodb.net/hrm?retryWrites=true&w=majority",
        urlencoding::encode("pass@123")
    );

    // Parse the MongoDB connection URI
    let client_options = ClientOptions::parse(db_uri.as_str()).await?;

    // Optionally configure client options (e.g., SSL settings, connection pool options)

    // Connect to the MongoDB server
    let client = Client::with_options(client_options)?;

    // Access a specific database
    let db = client.database("hrm");

    // Use `db` to access collections and perform operations

    // println!("Successfully connected to MongoDB!");


    let collection = db.collection::<Document>("BioMetricActivity");

    Ok(collection)
}

pub async fn get_mongodb_user_data_collection() -> Result<Collection<Document>, Box<dyn Error>> {
    // MongoDB connection URI
    let db_uri = format!(
        "mongodb+srv://pdtuae-hrm:{}@cluster0.mf10u.mongodb.net/hrm?retryWrites=true&w=majority",
        urlencoding::encode("pass@123")
    );

    // Parse the MongoDB connection URI
    let client_options = ClientOptions::parse(db_uri.as_str()).await?;

    // Optionally configure client options (e.g., SSL settings, connection pool options)

    // Connect to the MongoDB server
    let client = Client::with_options(client_options)?;

    // Access a specific database
    let db = client.database("hrm");

    // Use `db` to access collections and perform operations

    // println!("Successfully connected to MongoDB!");


    let collection = db.collection::<Document>("BioMetricUserData");

    Ok(collection)
}


