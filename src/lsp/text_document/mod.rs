use serde::{Deserialize, Serialize};

pub mod did_open;
pub mod did_change;
pub mod hover;
pub mod definition;

type DocumentUri = String;

#[derive(Deserialize)]
/// An item to transfer a text document from the client to the server.
pub struct TextDocumentItem {
    /// The text document's URI.
    pub uri: DocumentUri,
    /// The text document's language identifier.
    #[serde(rename = "languageId")]
    pub language_id: String,
    ///  The version number of this document (it will increase after each change, including undo/redo).
    pub version: i32,
    /// The content of the opened text document.
    pub text: String,
}

#[derive(Deserialize)]
pub struct TextDocumentIdentifier {
    pub uri: String,
}

#[derive(Deserialize)]
pub struct VersionedTextDocumentIdentifier {
    #[serde(flatten)]
    pub identifier: TextDocumentIdentifier,
    pub version: i32,
}

#[derive(Deserialize)]
pub struct TextDocumentPositionParams {
    #[serde(rename = "textDocument")]
    pub text_document: TextDocumentIdentifier,
    pub position: Position,
}

#[derive(Serialize, Deserialize)]
pub struct Position {
    /// Line position in a document (zero-based).
    pub line: u32,
    pub character: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Location {
    pub uri: DocumentUri,
    pub range: Range,
}

#[derive(Serialize, Deserialize)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}