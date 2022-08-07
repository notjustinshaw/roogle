use std::collections::HashMap; 
use std::fmt::{Debug, Display};

/// A bidirectional mapping of document names to document IDs.
///
/// A document ID is a unique, unsigned integer used to refer to a document in
/// a more compact form than a document name.
pub struct DocTable {
    pub(crate) name_to_id: HashMap<String, usize>,
    pub(crate) id_to_name: HashMap<usize, String>,
}

impl DocTable {
    /// Creates a new empty DocTable.
    pub fn new() -> Self {
        Self {
            name_to_id: HashMap::new(),
            id_to_name: HashMap::new(),
        }
    }

    /// Adds a new document to the DocTable.
    pub fn num_docs(&self) -> usize {
        self.name_to_id.len()
    }

    /// Adds a new document to the DocTable.
    ///
    /// The document is added to the DocTable with a new unique ID.
    pub fn add(&mut self, doc: &str) -> usize {
        let id = self.name_to_id.len();
        self.name_to_id.insert(doc.to_string(), id);
        self.id_to_name.insert(id, doc.to_string());
        id
    }

    /// Returns the ID of a document.
    ///
    /// The ID is returned if the document is in the DocTable.
    pub fn get_id(&self, doc: &str) -> Option<usize> {
        self.name_to_id.get(doc).map(|id| *id)
    }

    /// Returns the document name of a document ID.
    ///
    /// The document name is returned if the document ID is in the DocTable.
    pub fn get_name(&self, id: usize) -> Option<&str> {
        self.id_to_name.get(&id).map(|doc| doc.as_str())
    }
}

impl Display for DocTable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:#?}", self.name_to_id)
    }
}

impl Debug for DocTable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.name_to_id)
    }
}
