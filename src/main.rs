use actix_files as fs;
use actix_web::{web, App, HttpServer, HttpResponse, http::header,HttpRequest, http::StatusCode , Result};
use serde::{Serialize,Deserialize};
use std::fs::{OpenOptions, File};
use std::io::Write;
use std::io::{self, BufRead};
use actix_web::middleware::Logger;
use dotenv::dotenv;
use std::env;

#[derive(Serialize, Deserialize)]
pub struct FormData {
    nome: String,
    email: String,
    assunto: String,
    texto: String,
}

#[derive(Serialize)]
struct FormSubmission {
    id: usize,
    data: FormData,
}

async fn handle_form(form: web::Form<FormData>) -> Result<HttpResponse, actix_web::Error> {
    let serialized_data = serde_json::to_string(&form)?;

    // Open the file with write mode and append data if the file already exists
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("contactos.txt");

    match file {
        Ok(mut file) => {
            // Write the serialized data to the file followed by a newline
            writeln!(file, "{}", serialized_data)?;
            Ok(HttpResponse::Found()
                .status(StatusCode::FOUND)
                .append_header((header::LOCATION, "/"))
                .finish())
        }
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            Err(actix_web::error::ErrorInternalServerError("Internal Server Error"))
        }
    }
}


async fn get_submissions(req: HttpRequest) -> Result<HttpResponse> {
    let file_path = "contactos.txt";
    let secret_token = env::var("SECRET_TOKEN").expect("SECRET_TOKEN not found in .env");
    if let Some(token) = req.headers().get("Authorization") {
        if token.to_str().unwrap() ==  secret_token{
            let file = File::open(file_path)?;

            let submissions: Vec<FormSubmission> = io::BufReader::new(file)
                .lines()
                .enumerate()
                .filter_map(|(id, line)| {
                    match serde_json::from_str::<FormData>(&line.unwrap()) {
                        Ok(data) => Some(FormSubmission { id, data }),
                        Err(_) => None, // Skip invalid JSON lines
                    }
                })
                .collect();

            Ok(HttpResponse::Ok().json(submissions))
        } else {
            // Invalid token
            Ok(HttpResponse::Unauthorized().finish())
        }
    } else {
        // Token not provided
        Ok(HttpResponse::Unauthorized().finish())
    }
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Start Actix server
    
    dotenv().ok();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/submit_form", actix_web::web::post().to(handle_form))
            .route("/submissions", web::get().to(get_submissions))
            .service(fs::Files::new("/", "frontend").index_file("homepage.html"))
    })
    .bind(("0.0.0.0", 5555))?
    .run()
    .await
}