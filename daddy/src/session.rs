use crate::Client;
use std::{default::Default, io::Read};

use base64::{engine::general_purpose, Engine as _};

use serde::Serialize;

use crate::{API_URL, OpenOptions};
use crate::{HeaderMap, HeaderValue};

use reqwest::header::AUTHORIZATION;

#[derive(Default, Debug, Serialize)]
pub struct SessionToken {
    pub value: String,
    pub exp: u64
}
impl SessionToken {
    pub fn new(raw: String) -> Self {
        let mut base = Self::default();

        let payload: serde_json::Value = serde_json::from_str(&raw)
            .unwrap(); 
        let token = payload["token"].as_str().unwrap().to_string();

        base.value = token;
        base.exp = Self::decode_expiry(&payload["token"].to_string()).unwrap();

        return base
    }
    fn decode_expiry(raw_payload: &str) -> Result<u64, Box<dyn std::error::Error>> {
        let pieces: Vec<&str> = raw_payload.split(".")
            .collect();

        // UTF-8 
        
        let bytes = general_purpose::URL_SAFE_NO_PAD.decode(pieces[1].trim())?;
        let payload = String::from_utf8(bytes)?;

        
        let json_payload: serde_json::Value = serde_json::from_str(&payload)?;
        let exp = json_payload["exp"].to_string();

        Ok(exp.parse().unwrap())
    }
    pub fn from_creds(name: String) -> std::io::Result<Self> {
        let mut file = OpenOptions::new()
            .read(true)
            .open(name)?;
        let mut content = String::new();

        let _ = file.read_to_string(&mut content)?;

        let json_value: serde_json::Value = serde_json::from_str(&content)
            .unwrap();

        Ok(Self {
            value: json_value["value"].as_str().unwrap().into(),
            exp: json_value["exp"].as_u64().unwrap()
        })
    }
    pub fn to_header(&self) -> HeaderMap {
        let bearer = format!("Bearer {}", &self.value);
        let mut headers = HeaderMap::new();

        headers.insert(AUTHORIZATION, HeaderValue::from_str(&bearer).unwrap());
        return headers
    }
}
pub async fn create_session_token(client: Client) -> reqwest::Result<SessionToken> {
    let url = format!("{}/v2/auth/temporary", API_URL);
    let text = client.get(url)
        .send()
        .await?
        .text()
        .await?;

    Ok(text_to_token(text))
}

fn text_to_token(response: String) -> SessionToken {
    let json_value: serde_json::Value = serde_json::from_str(&response)
        .unwrap_or_default();

    SessionToken::new(json_value.to_string())
}