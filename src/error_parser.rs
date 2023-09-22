use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub method: String,
    pub id: i64,
    pub env: String,
    pub error: Error,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Error {
    pub code: i64,
    pub message: String,
}