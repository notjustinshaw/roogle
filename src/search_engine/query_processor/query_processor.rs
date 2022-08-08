use crate::{
    search_engine::crawler::{crawler::Crawler, fs_crawler::FileSystemCrawler},
    search_engine::indexer::{doc_table::DocTable, mem_index::MemIndex},
};

use super::{
    intersect::Intersect,
    query_result::QueryResult,
    query_token::{query_to_tokens, QueryToken},
};

/// Processes queries using inverted indices.
pub struct QueryProcessor {
    pub(crate) doc_table: DocTable,
    pub(crate) mem_index: MemIndex,
    stop_words: bool,
}

impl QueryProcessor {
    /// Creates a new query processor.
    pub fn new(root: &str, stop_words: bool) -> Self {
        let crawler = FileSystemCrawler::new(root, stop_words);
        let (doc_table, mem_index) = crawler.crawl().expect("failed to crawl");
        Self {
            doc_table,
            mem_index,
            stop_words,
        }
    }

    /// The number of documents in the index.
    pub fn num_docs(&self) -> usize {
        self.doc_table.num_docs()
    }

    /// The number of terms in the index.
    pub fn num_terms(&self) -> usize {
        self.mem_index.num_terms()
    }

    /// Searches the index for documents matching the query.
    ///
    /// Returns a vector of QueryResults sorted by rank that match the given
    /// query. To match the query, a document must contain all of the terms in
    /// the query. The rank of a document is the number of terms that it
    /// contains that are also in the query.
    ///
    /// For example, if the query is `steve "the hair" hairington`, then the
    /// search results will contain only documents that contain:
    /// * the term "steve"
    /// * the phrase "the hair" (the term "the" followed by the term "hair")
    /// * the term "hairington"
    ///
    /// A phrase is matched based on the positions of each term in the phrase.
    pub fn search(&self, query: &str) -> Vec<QueryResult> {
        let tokens: Vec<QueryToken> = query_to_tokens(query, self.stop_words);

        // Search for each token individually.
        let mut meta_results: Vec<Vec<QueryResult>> = Vec::new();
        for token in tokens.iter() {
            meta_results.push(token.search(&self.mem_index, &self.doc_table));
        }

        // Intersect the results.
        let itr = meta_results.iter_mut();
        if let Some(results) = itr.reduce(|acc, next| {
            acc.intersect(next);
            acc
        }) {
            // Sort the results by rank (highest to lowest) then return them.
            results.sort_by(|a, b| b.cmp(a));
            results.to_vec()
        } else {
            Vec::new()
        }
    }
}
