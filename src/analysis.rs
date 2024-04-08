
use std::collections::HashMap;

use crate::lsp::text_document::definition::DefinitionResponse;
use crate::lsp::text_document::hover::{HoverResponse, HoverResult};
use crate::lsp::message::Response;
use crate::lsp::text_document::{Location, Position, Range};

pub struct State {
    /// Map of file names(uri) to contents
    pub documents: HashMap<String, String>,
}

impl State {
    pub fn new() -> Self {
        State {
            documents: HashMap::new(),
        }
    }

    pub fn open_document(&mut self, uri: String, text: String) {
        self.documents.insert(uri, text);
    }

    /// Panics if document does not exist
    pub fn update_document(&mut self, uri: &str, text: String) {
        let document = self.documents.get_mut(uri).unwrap();
        *document = text;
    }

    /// Panics of document does not exist
    pub fn hover(&self, id: i32, uri: String) -> HoverResponse {
        let document = &self.documents[&uri];

        HoverResponse {
            response: Response {
                rpc: "2.0".into(),
                id: Some(id),
            },
            result: HoverResult {
                contents: format!("File: {}, Characters: {}", uri, document.len()),
            },
        }
    }

    pub fn definition(&self, id: i32, uri: &str, position: Position) -> DefinitionResponse {
        DefinitionResponse {
            response: Response {
                rpc: "2.0".into(),
                id: Some(id),
            },
            result: Location {
                uri: uri.into(),
                range: Range {
                    start: Position {
                        line: position.line.saturating_sub(1),
                        character: 0,
                    },
                    end: Position {
                        line: position.line.saturating_sub(1),
                        character: 0,
                    },
                },
            },
        }
    }
}