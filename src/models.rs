use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

#[derive(Debug, Serialize, Clone)]
pub struct AppState {
    pub app_name: String,
    pub app_version: String,
}

#[derive(Debug, Serialize, Deserialize)]
//TODO: change ::from to -> #[pg_mapper(table = "users")]
pub struct User {
    pub id: Option<String>,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
}

impl User {
    pub fn from(row: &Row) -> Self {
        User {
            id: row.get("id"),
            email: row.get("email"),
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
            username: row.get("username"),
        }
    }
}
