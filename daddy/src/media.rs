use crate::{Client, SessionToken, API_URL};

use serde::Serialize;
use colored::*;

#[derive(Serialize)]
pub struct Content {
    pub user: String,
    pub video_url: String
}
pub async fn fetch_contents(client: Client, suburl: &str) -> Result<Vec<Content>, Box<dyn std::error::Error>> {
    let token = SessionToken::from_creds("daddy-0.0.1/creds.json".into())?;
    let url = format!("{API_URL}{}", suburl);

    let response = client.get(url)
        .headers(token.to_header())
        .send()
        .await?
        .error_for_status()?;

    Ok(parse_contents(response.text().await?)?)
}
fn parse_contents(raw: String) -> serde_json::Result<Vec<Content>> {
    println!("[!] Populating the media queries\n");
    let json_value: serde_json::Value = serde_json::from_str(&raw)?;
    let mut contents = Vec::new();
    
    let array = json_value.get("gifs").or_else(|| json_value.get("bestGifs"));
    for content in array.unwrap().as_array().unwrap() {
        let user = content["userName"].as_str().unwrap();
        contents.push(Content {user: user.into(), 
        video_url: content["urls"]["sd"].as_str().unwrap().into()});
        
        println!("{}[+] {} {}'s content", " ".repeat(4), "Added".green(), user);
    }

    Ok(contents)
}
