use actix_cors::Cors;
use actix_files::Files;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use serde::Deserialize;
use serde::Serialize;
use std::env;

#[derive(Deserialize)]
struct MyForm {
    text: String,
}

#[derive(Serialize)]
struct OpenAIToSpeechRequest {
    input: String,
    voice: String, // Specify the desired voice
    model: &'static str,
}

async fn generate_audio_from_text(text: &str) -> Result<Vec<u8>, reqwest::Error> {
    dotenv().ok(); // Load environment variables from .env file

    let openai_api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set"); // Get the API key from environment variables
    let openai_api_url = "https://api.openai.com/v1/audio/speech";
    let request_body = OpenAIToSpeechRequest {
        input: text.to_string(),
        voice: "onyx".to_string(), // Replace with the desired voice
        model: "tts-1",
    };

    let client = reqwest::Client::new();
    let response = client
        .post(openai_api_url)
        .header("Authorization", format!("Bearer {}", openai_api_key))
        .json(&request_body)
        .send()
        .await?;

    let audio_data = response.bytes().await?;

    Ok(audio_data.to_vec())
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
