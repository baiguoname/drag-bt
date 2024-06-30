use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FileResp {
    pub content: String,
}