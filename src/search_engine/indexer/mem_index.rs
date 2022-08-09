use std::collections::HashMap;
use std::fmt::{Debug, Display};

use super::doc_index::DocIndex;

/// An in-memory inverted index.
///
/// A MemIndex is a combination of many smaller document indexes so that a
/// search can be performed across multiple documents. The MemIndex is a
/// map from a term to a map of document IDs to positions in the document.
pub struct MemIndex {
    pub(crate) index: HashMap<String, HashMap<usize, Vec<usize>>>,
}

impl MemIndex {
    /// Creates a new empty MemIndex.
    pub fn new() -> Self {
        Self {
            index: HashMap::new(),
        }
    }

    /// Returns the number of terms in the index.
    pub fn num_terms(&self) -> usize {
        self.index.len()
    }

    /// Adds a new document to the MemIndex.
    pub fn add(&mut self, mut doc_index: DocIndex, doc_id: usize) {
        for (term, positions) in doc_index.index.drain() {
            self.index
                .entry(term)
                .or_insert(HashMap::new())
                .insert(doc_id, positions);
        }
    }

    /// Searches the MemIndex for a given term.
    pub fn search(&self, term: &str) -> Option<&HashMap<usize, Vec<usize>>> {
        self.index.get(term)
    }
}

impl Display for MemIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:#?}", self.index)
    }
}

impl Debug for MemIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.index)
    }
}
