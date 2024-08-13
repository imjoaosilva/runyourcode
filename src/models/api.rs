use serde::{Deserialize, Serialize};


#[derive(Serialize)]
pub struct RequestBody {
    pub command: String,
    pub files: serde_json::Value,
    pub sandbox: String,
    pub version: String,
}

#[derive(Serialize, Deserialize)]
pub struct Files {
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

#[derive(Serialize)]
pub struct RustExpand {
    pub edition: String,
    pub code: String,
}

#[derive(Deserialize)]
pub struct RustExpandResponse {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}