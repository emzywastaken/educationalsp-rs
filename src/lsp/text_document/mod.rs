use serde::Deserialize;

pub mod did_open;
pub mod did_change;

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