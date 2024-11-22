use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::process;
use std::path::Path;

#[derive(Serialize)]
struct TranslateRequest {
    source_code: String,
    target_language: Option<String>,
}

#[derive(Deserialize)]
struct TranslateResponse {
    translated_code: String,
}

#[tokio::main]
async fn main() {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: client <source_code_or_file_path> <target_language>");
        process::exit(1);
    }

    let source = &args[1];
    let target_language = &args[2];

    // Determine if the source is a file path or code
    let source_code = if Path::new(source).exists() {
        // Read the file content
        match fs::read_to_string(source) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error reading the file: {}", e);
                process::exit(1);
            }
        }
    } else {
        // Use the source as code
        source.to_string()
    };

    // Get the server URL from the environment variable or use the default
    let server_url = env::var("SERVER_URL")
        .unwrap_or_else(|_| "http://localhost:3030/translate".to_string());

    // Create the client
    let client = Client::new();

    // Create the request body
    let request_body = TranslateRequest {
        source_code,
        target_language: Some(target_language.to_string()),
    };

    // Send the POST request
    match client.post(&server_url)
        .json(&request_body)
        .send()
        .await {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<TranslateResponse>().await {
                    Ok(translated_response) => {
                        println!("{}", translated_response.translated_code);
                    }
                    Err(e) => {
                        eprintln!("Error parsing response: {}", e);
                    }
                }
            } else {
                eprintln!("Server returned an error: {}", response.status());
            }
        }
        Err(e) => {
            eprintln!("Error sending request: {}", e);
        }
    }
}

