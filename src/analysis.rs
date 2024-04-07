
use std::collections::HashMap;

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
}