use std::cmp::Ordering;
use std::fmt::{Debug, Display};

use crate::search_engine::indexer::doc_table::DocTable;

/// A result of a query.
///
/// Contains the name and id of the document and its rank. The rank is the
/// number of terms in the query that are also in the document.
#[derive(Clone, Eq)]
pub struct QueryResult {
    pub doc_id: usize,
    pub doc_name: String,
    pub rank: usize,
}

impl QueryResult {
    /// Creates a new QueryResult.
    pub fn new(doc_id: usize, doc_name: String, rank: usize) -> Self {
        Self {
            doc_id,
            doc_name,
            rank,
        }
    }

    /// Creates a new QueryResult from the docid and the rank.
    pub fn from(doc_id: usize, postings: &Vec<usize>, doc_table: &DocTable) -> Self {
        let maybe_name = doc_table.get_name(doc_id);
        let name = maybe_name.expect("doc_id not found").to_string();
        let roodrank = postings.len();
        Self {
            doc_id,
            doc_name: name,
            rank: roodrank,
        }
    }
}

impl PartialEq for QueryResult {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank && self.doc_id == other.doc_id
    }
}

impl Ord for QueryResult {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.rank.cmp(&other.rank) {
            Ordering::Equal => self.doc_id.cmp(&other.doc_id),
            other => other,
        }
    }
}

impl PartialOrd for QueryResult {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.rank.cmp(&other.rank) {
            Ordering::Equal => Some(self.doc_id.cmp(&other.doc_id)),
            other => Some(other),
        }
    }
}

impl Display for QueryResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}] {} {}", self.doc_id, self.doc_name, self.rank)
    }
}

impl Debug for QueryResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}] {} {}", self.doc_id, self.doc_name, self.rank)
    }
}
