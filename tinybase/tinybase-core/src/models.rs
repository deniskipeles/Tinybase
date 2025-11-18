use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub struct Collection {
    pub name: String,
}

#[derive(Deserialize)]
pub struct Record {
    pub data: Value,
}
