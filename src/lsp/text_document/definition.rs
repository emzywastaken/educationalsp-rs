use serde::{Deserialize, Serialize};

use crate::lsp::message::{Request, Response};

use super::{Location, TextDocumentPositionParams};

#[derive(Deserialize)]
pub struct DefinitionRequest {
    #[serde(flatten)]
    pub request: Request,
    pub params: DefinitionParams,
}

#[derive(Deserialize)]
pub struct DefinitionParams {
    #[serde(flatten)]
    pub text_document_position_params: TextDocumentPositionParams,
}

#[derive(Serialize, Deserialize)]
pub struct DefinitionResponse {
    #[serde(flatten)]
    pub response: Response,
    pub result: Location,
}