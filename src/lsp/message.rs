use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Request {
    #[serde(rename = "jsonrpc")]
    pub rpc: String,
    pub id: i32,
    pub method: String,
    // we will just specify the type of the params in all of the request types later
    // pub params: ?
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    #[serde(rename = "jsonrpc")]
    pub rpc: String,
    pub id: Option<i32>,
    // Result,
    // Error,
}

#[derive(Deserialize)]
pub struct Notification {
    #[serde(rename = "jsonrpc")]
    pub rpc: String,
    pub method: String,
}
