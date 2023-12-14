use crate::utils;

use mongodb::{Client, Collection, Database};
use utils::SentimentResult;

// Define your struct to represent data stored in MongoDB
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SentimentData {
    pub input: String,
    pub sentiment: SentimentResult,
}

pub async fn get_mongo_collection() -> Collection<SentimentData> {
    let client_uri = "mongodb://rootuser:rootpass@localhost:27017";
    let client = Client::with_uri_str(client_uri)
        .await
        .expect("Failed to connect to MongoDB");

    let db: Database = client.database("sentiment_analysis");
    db.collection::<SentimentData>("input_data")
}
