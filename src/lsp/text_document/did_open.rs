use serde::Deserialize;

use crate::lsp::message::Notification;

use super::TextDocumentItem;

#[derive(Deserialize)]
pub struct DidOpenTextDocumentNotification {
    #[serde(flatten)]
    pub notification: Notification,
    pub params: DidOpenTextDocumentParams,
}

#[derive(Deserialize)]
pub struct DidOpenTextDocumentParams {
    #[serde(rename = "textDocument")]
    pub text_document: TextDocumentItem,
}
