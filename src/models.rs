use serde::{Deserialize, Serialize};

use crate::constants::{STATUS_DONE, STATUS_IN_PROGRESS, STATUS_TODO};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Status {
    Todo,
    InProgress,
    Done,
}

impl Status {
    /// Devuelve la representacion textual del estado para salida por consola.
    pub fn as_str(&self) -> &'static str {
        match self {
            Status::Todo => STATUS_TODO,
            Status::InProgress => STATUS_IN_PROGRESS,
            Status::Done => STATUS_DONE,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: u32,
    pub description: String,
    #[serde(default)]
    pub user_id: u32,
    pub status: Status,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: u32,
    pub name: String,
    pub created_at: String,
}