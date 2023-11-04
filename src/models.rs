use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone)]
pub struct AppState {
    pub app_name: String,
    pub app_version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Option<String>,
    pub name: String,
    pub age: u8,
}
