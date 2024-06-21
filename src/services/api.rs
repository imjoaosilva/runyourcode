use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize)]
struct RequestBody {
    command: String,
    files: serde_json::Value,
    sandbox: String,
    version: String,
}

#[derive(Serialize, Deserialize)]
struct Files {
    pub field: String,
}

#[derive(Deserialize)]
pub struct ResponseBody {
    pub id: String,
    pub ok: bool,
    pub duration: u64,
    pub stdout: String,
    pub stderr: String,
}

pub async fn run_code(language: String, code: String) -> Result<ResponseBody, String> {
    let body = RequestBody {
        command: String::from("run"),
        files: json!({
            "": code
        }),
        sandbox: language,
        version: String::new(),
    };

    let client = reqwest::Client::new();
    let response = client
        .post("https://api.codapi.org/v1/exec")
        .json(&body)
        .send()
        .await
        .unwrap();
    
    if response.status().is_success() {
        
        let response: ResponseBody = response.json().await.unwrap();
        Ok(response)
    } else {
        Err("Failed to execute code!".to_string())
    }
    
}