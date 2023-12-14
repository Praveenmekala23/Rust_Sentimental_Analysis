mod db;
mod utils;

use rocket::{http::Status, response::content::RawHtml};

use db::SentimentData;
use rocket::serde::json::{json, Value};
use sentiment::analyze;
use tera::Tera;
use utils::{parse_csv_to_struct, read_sentiment140_dataset, SentimentResult};

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> Result<String, std::io::Error> {
    let file_path = "sample.csv";
    let contents = read_sentiment140_dataset(file_path)?;

    let json_data = parse_csv_to_struct(&contents);
    Ok(json_data)
}

#[post("/", data = "<input_data>")]
fn process_data(input_data: String) -> String {
    format!("Received data: {}", input_data)
}

#[post("/", data = "<text>")]
async fn sentiment_analysis(text: String) -> Value {
    let mut sanitized_text = String::new();
    let mut sentiment_result = SentimentResult {
        positive: 0.0,
        negative: 0.0,
        score: 0.0,
        comparative: 0.0,
    };

    let text_json: Value = serde_json::from_str(&text).unwrap_or_else(|_| json!({}));

    if let Some(json_text) = text_json.get("text") {
        if let Some(text_str) = json_text.as_str() {
            sanitized_text = text_str.to_string();

            if sanitized_text.trim().is_empty() {
                sanitized_text = "Input text is Empty!".to_string();
            } else {
                sentiment_result = SentimentResult {
                    positive: analyze(sanitized_text.clone()).positive.score,
                    negative: analyze(sanitized_text.clone()).negative.score,
                    score: analyze(sanitized_text.clone()).score,
                    comparative: analyze(sanitized_text.clone()).comparative,
                };
            }
        }
    }

    let json_response = json!({
        "input": sanitized_text,
        "sentiment": sentiment_result,
    });

    // Convert json_response to SentimentData
    let sentiment_data = SentimentData {
        input: json_response["input"].to_string(),
        sentiment: SentimentResult {
            positive: json_response["sentiment"]["positive"]
                .as_f64()
                .unwrap_or(0.0) as f32,
            negative: json_response["sentiment"]["negative"]
                .as_f64()
                .unwrap_or(0.0) as f32,
            score: json_response["sentiment"]["score"].as_f64().unwrap_or(0.0) as f32,
            comparative: json_response["sentiment"]["comparative"]
                .as_f64()
                .unwrap_or(0.0) as f32,
        },
    };

    let collection = db::get_mongo_collection().await;
    if let Err(e) = collection.insert_one(sentiment_data, None).await {
        eprintln!("Failed to insert document: {}", e);
    } else {
        print!("Inserted document to DB: {}", json_response.clone());
    }

    json_response
}

#[get("/")]
fn display() -> Result<RawHtml<String>, Status> {
    let mut context = tera::Context::new();

    context.insert("text", "input");
    context.insert("sentiment", "sentiment");

    let tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error parsing templates: {}", e);
            return Err(Status::InternalServerError);
        }
    };

    let rendered = match tera.render("index.html", &context) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error rendering template: {}", e);
            return Err(Status::InternalServerError);
        }
    };

    Ok(RawHtml(rendered))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/data", routes![process_data])
        .mount("/sentiment", routes![sentiment_analysis])
        .mount("/display", routes![display])
}
