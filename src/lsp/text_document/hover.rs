use serde::{Deserialize, Serialize};

use crate::lsp::message::{Request, Response};

use super::TextDocumentPositionParams;

#[derive(Deserialize)]
pub struct HoverRequest {
    #[serde(flatten)]
    pub request: Request,
    pub params: HoverParams,
}

#[derive(Deserialize)]
pub struct HoverParams {
    #[serde(flatten)]
    pub text_document_position_params: TextDocumentPositionParams,
}

#[derive(Serialize, Deserialize)]
pub struct HoverResponse {
    #[serde(flatten)]
    pub response: Response,
    pub result: HoverResult,
}


#[derive(Serialize, Deserialize)]
pub struct HoverResult {
    pub contents: String,
}