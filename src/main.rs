use actix_cors::Cors;
use actix_files::Files;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::env;

#[derive(Deserialize)]
struct MyForm {
    text: String,
}

#[derive(Serialize)]
struct OpenAIMessage {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct OpenAICompletionRequest {
    messages: Vec<OpenAIMessage>,
    model: &'static str,
}

#[derive(Serialize)]
struct OpenAIToSpeechRequest {
    input: String,
    voice: String, // Specify the desired voice
    model: &'static str,
}

#[derive(Debug)]
struct ParsedMessage {
    voice: String,
    message: String,
}

fn parse_response(response_body: &str) -> Vec<ParsedMessage> {
    let v: Value = serde_json::from_str(response_body).expect("JSON was not well-formatted");

    let messages = &v["choices"][0]["message"]["content"]
        .as_str()
        .expect("Content is not a string");

    let mut speaker = String::new();
    let mut results = Vec::new();

    for line in messages.lines() {
        if line.starts_with("Speaker 1:") {
            speaker = "onyx".to_string();
            results.push(ParsedMessage {
                voice: speaker.clone(),
                message: line.replacen("Speaker 1: ", "", 1), // Remove "Speaker 1: " prefix
            });
        } else if line.starts_with("Speaker 2:") {
            speaker = "nova".to_string();
            results.push(ParsedMessage {
                voice: speaker.clone(),
                message: line.replacen("Speaker 2: ", "", 1), // Remove "Speaker 2: " prefix
            });
        }
    }

    results
}

async fn generate_audio_from_text(text: &str) -> Result<Vec<u8>, reqwest::Error> {
    dotenv().ok(); // Load environment variables from .env file

    if env::var("OPENAI_API_KEY").is_err() {
        panic!("OPENAI_API_KEY must be set"); // Panic if the API key is not set
    }

    let openai_api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set"); // Get the API key from environment variables

    // convert the text to a 2 speakers podcast transcript using gpt
    let openai_api_completion = "https://api.openai.com/v1/chat/completions";
    let completion_body = OpenAICompletionRequest {
        messages: vec![
            OpenAIMessage {
                role: "system".to_string(),
                content: "As a text-to-podcast assistant, your job is to transform text into a dynamic two-speaker podcast dialogue.
                Craft distinct speaker personalities with unique speaking styles and adapt the tone to fit the topic.
                Make the conversation lively with natural interruptions, emotional expressions, humor, and audience engagement.
                The output should be a string array, each marking the switch between speakers, to create an engaging and realistic podcast experience.
                Use capitalization and punctuation to make the conversation engaging. And you are very focused on transforming the text into something pedagogical, engaging, and into a story.".to_string(),
            },
            OpenAIMessage {
                role: "user".to_string(),
                content: text.to_string(),
            }
        ],
        model: "gpt-3.5-turbo-16k",
    };

    // Send the request to the OpenAI API
    let client = reqwest::Client::new();
    let response = client
        .post(openai_api_completion)
        .header("Authorization", format!("Bearer {}", openai_api_key))
        .json(&completion_body)
        .send()
        .await?;

    // Handle the response from the OpenAI API
    let response_body = response.text().await?;
    println!("Response: {}", response_body);

    let parsed_messages = parse_response(&response_body);
    let mut audio_data_combined = Vec::new();

    for parsed_message in parsed_messages {
        let openai_api_speech = "https://api.openai.com/v1/audio/speech";
        let request_body = OpenAIToSpeechRequest {
            input: parsed_message.message,
            voice: parsed_message.voice,
            model: "tts-1",
        };

        let response = client
            .post(openai_api_speech)
            .header("Authorization", format!("Bearer {}", openai_api_key))
            .json(&request_body)
            .send()
            .await?;

        let audio_data = response.bytes().await?;
        audio_data_combined.extend(audio_data); // Combine audio data
    }

    Ok(audio_data_combined)
}

async fn create_podcast(form: web::Json<MyForm>) -> impl Responder {
    let text = &form.text;

    match generate_audio_from_text(text).await {
        Ok(audio_data) => {
            HttpResponse::Ok()
                .content_type("audio/mpeg") // Or the correct MIME type for your audio data
                .body(audio_data)
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to generate audio"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .wrap(Logger::default())
            // .route("/create-podcast", web::post().to(create_podcast))
            .service(web::resource("/create-podcast").route(web::post().to(create_podcast)))
            .service(Files::new("/", "./static").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .workers(2)
    .run()
    .await
}
