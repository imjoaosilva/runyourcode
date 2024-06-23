use crate::{
    models::api::{RequestBody, ResponseBody, RustExpand, RustExpandResponse},
    utils::util::extract_relevant_lines,
};
use serde_json::json;

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
        let mut response: ResponseBody = response.json().await.unwrap();

        if response.stdout == "" {
            response.stdout = "Nothing was returned".to_string();
        };
        Ok(response)
    } else {
        Err("Failed to execute code!".to_string())
    }
}

pub async fn rust_expand(edition: String, code: String) -> Result<RustExpandResponse, String> {
    let body = RustExpand { edition, code };

    let client = reqwest::Client::new();
    let response = client
        .post("https://play.rust-lang.org/macro-expansion")
        .json(&body)
        .send()
        .await
        .unwrap();

    if response.status().is_success() {
        let mut result: RustExpandResponse = response.json().await.unwrap();

        result.stdout = extract_relevant_lines(
            &result.stdout,
            &["Finished ", "Compiling playground"],
            &["error: aborting"],
        ).to_owned();

        Ok(result)
    } else {
        Err("Failed to expand code!".to_string())
    }
}
