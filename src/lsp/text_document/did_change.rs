use serde::Deserialize;

use crate::lsp::message::Notification;

use super::VersionedTextDocumentIdentifier;

#[derive(Deserialize)]
pub struct TextDocumentDidChangeNotification {
    #[serde(flatten)]
    pub notification: Notification,
    pub params: DidChangeTextDocumentParams,
}

#[derive(Deserialize)]
pub struct DidChangeTextDocumentParams {
    #[serde(rename = "textDocument")]
    pub text_document: VersionedTextDocumentIdentifier,
    #[serde(rename = "contentChanges")]
    pub content_changes: Vec<TextDocumentContentChangeEvent>,
}

#[derive(Deserialize)]
pub struct TextDocumentContentChangeEvent {
    // The new text of the whole document.
    pub text: String,
}