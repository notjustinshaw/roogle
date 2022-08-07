use std::cmp::Ordering;

/// A result of a query.
/// 
/// Contains the name and id of the document and its rank. The rank is the
/// number of terms in the query that are also in the document.
#[derive(Eq)]
pub struct QueryResult {
    pub doc_id: usize,
    pub doc_name: String,
    pub rank: usize,
}

impl QueryResult {
    /// Creates a new QueryResult.
    pub fn new(doc_id: usize, doc_name: String, rank: usize) -> Self {
        Self { doc_id, doc_name, rank }
    }
}

impl PartialEq for QueryResult {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank
    }
}

impl Ord for QueryResult {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank.cmp(&other.rank)
    }
}

impl PartialOrd for QueryResult {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.rank.cmp(&other.rank))
    }
}
