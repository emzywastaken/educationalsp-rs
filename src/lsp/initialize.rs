use serde::{Deserialize, Serialize};

use super::message::{Request, Response};

#[derive(Deserialize)]
pub struct InitializeRequest {
    #[serde(flatten)]
    pub request: Request,
    pub params: InitializeRequestParams,
}

#[derive(Deserialize)]
pub struct InitializeRequestParams {
    #[serde(rename = "clientInfo")]
    pub client_info: Option<ClientInfo>,
    // ... theres tons more that goes here
}

#[derive(Deserialize)]
pub struct ClientInfo {
    pub name: String,
    pub version: String,
}

#[derive(Serialize, Deserialize)]
pub struct InitializeResponse {
    #[serde(flatten)]
    pub response: Response,
    pub result: InitializeResult,
}

impl InitializeResponse {
    pub fn new(id: i32) -> Self {
        Self {
            response: Response {
                rpc: "2.0".into(),
                id: Some(id),
            },
            result: InitializeResult {
                capabilities: ServerCapabilities {
                    text_document_sync: 1,
                    hover_provider: true,
                },
                server_info: ServerInfo {
                    name: "educationalsp-rs".into(),
                    version: "0.0.0.0-alpha2.final".into(),
                },
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct InitializeResult {
    capabilities: ServerCapabilities,
    #[serde(rename = "serverInfo")]
    server_info: ServerInfo,
}

#[derive(Serialize, Deserialize)]
struct ServerCapabilities {
    #[serde(rename = "textDocumentSync")]
    text_document_sync: i32,
    #[serde(rename = "hoverProvider")]
    hover_provider: bool,
}

#[derive(Serialize, Deserialize)]
struct ServerInfo {
    name: String,
    version: String,
}
