use contractions::Contractions;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SentimentResult {
    pub positive: f32,
    pub negative: f32,
    pub score: f32,
    pub comparative: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CsvRow {
    target: i64,
    ids: i64,
    date: String,
    flag: String,
    user: String,
    text: String,
}

pub fn read_sentiment140_dataset(file_path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(file_path)
}

pub fn parse_csv_to_struct(contents: &str) -> String {
    let mut json_array = Vec::new();

    for line in contents.lines().skip(1) {
        let fields: Vec<&str> = line.split(',').collect();

        if fields.len() >= 6 {
            let row = CsvRow {
                target: fields[0].trim_matches('"').parse().unwrap_or_default(),
                ids: fields[1].trim_matches('"').parse().unwrap_or_default(),
                date: fields[2].trim_matches('"').to_string(),
                flag: fields[3].trim_matches('"').to_string(),
                user: fields[4].trim_matches('"').to_string(),
                text: preprocess_text(fields[5].trim_matches('"')),
            };

            json_array.push(json!(row));
        }
    }

    serde_json::to_string_pretty(&json_array)
        .unwrap_or_else(|_| String::from("Error converting to JSON"))
}

pub fn preprocess_text(text: &str) -> String {
    let cleaned_text = clean_text(text);
    let normalized_text = normalize_text(&cleaned_text);
    normalized_text
}

pub fn clean_text(text: &str) -> String {
    let url_regex = Regex::new(r"https?://\S+").unwrap();
    let cleaned_text = url_regex.replace_all(text, "").to_string();

    let mention_regex = Regex::new(r"@\w+\s?").unwrap();
    let cleaned_text = mention_regex.replace_all(&cleaned_text, "").to_string();

    let cleaned_text = cleaned_text
        .replace("#", "")
        .replace("&amp;", "&")
        .replace("&gt;", ">")
        .replace("&lt;", "<")
        .replace("&quot;", "\"");

    cleaned_text
}

pub fn normalize_text(text: &str) -> String {
    let normalized_text = text.to_lowercase();

    let mut contractions = Contractions::new();
    let _ = contractions.add_from_json(contractions::EXPAND_SINGLE_CONTRACTIONS_JSON);

    contractions.apply(&normalized_text)
}
