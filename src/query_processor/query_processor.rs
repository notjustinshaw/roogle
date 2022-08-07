use crate::{
    crawler::{crawler::Crawler, fs_crawler::FileSystemCrawler},
    indexer::{doc_table::DocTable, mem_index::MemIndex},
};

use super::query_result::QueryResult;

/// Processes queries using inverted indices.
pub struct QueryProcessor {
    pub(crate) doc_table: DocTable,
    pub(crate) mem_index: MemIndex,
}

impl QueryProcessor {
    /// Creates a new query processor.
    pub fn new(root: &str) -> Self {
        let crawler = FileSystemCrawler::new(root);
        let (doc_table, mem_index) = crawler.crawl().expect("failed to crawl");
        Self {
            doc_table,
            mem_index,
        }
    }

    /// Searches the index for documents matching the query.
    ///
    /// Returns a vector of QueryResults sorted by rank that match the given
    /// query. To match the query, a document must contain all of the terms in
    /// the query. The rank of a document is the number of terms that it
    /// contains that are also in the query.
    ///
    /// For example, if the query is "oh my" and the index contains the document
    /// "My oh my!", then the document will be ranked with a rank of 3 since it
    /// matches the first query term once and the second query term twice.
    pub fn search(&self, query: &str) -> Vec<QueryResult> {
        let mut results: Vec<QueryResult> = Vec::new();
        let mut terms = query.split_whitespace();

        // Handle the first term in the query separately since we don't need to
        // filter out documents that don't match previous terms.
        if let Some(term) = terms.next() {
            self.handle_first_term(&mut results, term);
        }

        // Handle the remaining terms in the query.
        for term in terms {
            if let Some(new_results) = self.mem_index.search(term) {
                // Go through each result we have so far and filter out any
                // results that don't match the current term.
                results.retain_mut(|qr| {
                    if new_results.contains_key(&qr.doc_id) {
                        // Matches, keep it and adjust its rank.
                        let new_pos = new_results.get(&qr.doc_id).unwrap();
                        qr.rank += new_pos.len();
                        true
                    } else {
                        // Doesn't match, so remove it.
                        false
                    }
                });
            }
        }

        // Sort the results by rank (highest to lowest) then return them.
        results.sort_by(|a, b| b.cmp(a));
        results
    }

    /// Handles the first term in the query.
    ///
    /// By definition, all the documents that match the term will be added
    /// to the results.
    fn handle_first_term(&self, results: &mut Vec<QueryResult>, term: &str) {
        if let Some(search_results) = self.mem_index.search(term) {
            for (doc_id, positions) in search_results {
                let maybe_name = self.doc_table.get_name(*doc_id);
                let name = maybe_name.expect("doc_id not found").to_string();
                let roodrank = positions.len();
                let qr: QueryResult = QueryResult::new(*doc_id, name, roodrank);
                results.push(qr);
            }
        }
    }
}
