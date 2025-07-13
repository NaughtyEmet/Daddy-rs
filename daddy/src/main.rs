mod session;
mod media;

use session::{create_session_token, SessionToken};
use media::Content;

use reqwest::Client;
use reqwest::header::{USER_AGENT, HeaderValue, HeaderMap};

use std::fs::OpenOptions;
use std::io::Write;

use clap::{Subcommand, Parser};

use rand::Rng;

use media::fetch_contents;
const API_URL: &'static str = "https://api.redgifs.com";


#[derive(Parser)]
#[command(name = "Daddy")]
struct App {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    Spawn,
    Populate {
        #[arg(short)]
        typ: Option<String>
    },
    Goon,
    Stop
}

#[tokio::main]
async fn main() {
    let app = App::parse();

    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_str("daddy/0.0.1").unwrap());
    let client = Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();

    let _ = std::fs::create_dir("daddy-0.0.1");
    match &app.command {
        Commands::Spawn => {
            let token = create_session_token(client)
                .await
                .unwrap();

            save_token(token).unwrap();

            println!("[+] Successfully spawned a session");
        },
        Commands::Populate { typ } => {
            let some_contents;
            if let Some(typ) = typ {
                let genre = typ;
                println!("[!] Loading {genre} content");

                some_contents = fetch_contents(client, &format!("/v2/tags/feed/{genre}"))
                    .await;


                
            } else {
                println!("[!] Loading contents");
                some_contents = fetch_contents(client, "/v2/feeds/trending/popular?page=1&count=100")
                    .await;
            }

             match some_contents {
                    Ok(contents) => save_contents(contents).unwrap(),
                    Err(_) => eprintln!("[!] The session token has either expired or it hasn't been created")
            }
        },
        Commands::Goon => {
            let options = OpenOptions::new()
                .read(true)
                .open("daddy-0.0.1/media_queries.json");

            if let Ok(file) = options {
                let json_value: serde_json::Value = serde_json::from_reader(&file)
                    .unwrap();
                let arr = json_value.as_array()
                    .unwrap();

                let mut rng = rand::rng();
                let random: usize = rng.random_range(..arr.len());

                if webbrowser::open(arr[random]["video_url"].as_str().unwrap()).is_ok() {
                    println!("[+] Opening the browser for the goon session");
                } else {
                    eprintln!("[!] Couldn't open the browser :(");
                }
            } else {
                eprintln!("[!] media queries are missing")
            }
            

        },

        Commands::Stop => {
            if std::fs::exists("daddy-0.0.1").unwrap() {
                println!("[!] Removing all data");
                std::fs::remove_dir_all("daddy-0.0.1").unwrap();
            } else {
                eprintln!("[!] The directory doesn't exist");
            }
        }
    };
    
}

fn save_contents(contents: Vec<Content>) -> std::io::Result<()> {
    let json_token = serde_json::to_string_pretty(&contents)
        .unwrap();

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("daddy-0.0.1/media_queries.json")?;

    file.write_all(json_token.as_bytes())?;

    Ok(())
}
fn save_token(token: SessionToken) -> std::io::Result<()> {
    let json_token = serde_json::to_string_pretty(&token)
        .unwrap();


    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("daddy-0.0.1/creds.json")?;

    file.write_all(json_token.as_bytes())?;

    Ok(())
}
