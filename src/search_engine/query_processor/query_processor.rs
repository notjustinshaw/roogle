use crate::{
    search_engine::crawler::{crawler::Crawler, fs_crawler::FileSystemCrawler},
    search_engine::indexer::{doc_table::DocTable, mem_index::MemIndex},
};

use super::query_result::QueryResult;
use crate::search_engine::filters::stop_words::STOP_WORDS;

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
    /// * the phrase "the hair"
    /// * the term "hairington"
    ///
    /// A phrase is matched based on the positions of each term in the phrase.
    pub fn search(&self, query: &str) -> Vec<QueryResult> {
        let mut results: Vec<QueryResult> = Vec::new();
        let parsed_query: Vec<(&str, String)> = self.parse_query(query);
        let mut terms = parsed_query.iter();

        // Handle the first term in the query separately since we don't need to
        // filter out documents that don't match previous terms.
        if let Some((token_type, token)) = terms.next() {
            match *token_type {
                "term" => self.handle_leading_term(&mut results, token),
                "phrase" => self.handle_leading_phrase(&mut results, token),
                _ => panic!("unexpected token type"),
            }
        }

        // Handle the remaining terms in the query.
        for (token_type, token) in terms {
            match *token_type {
                "term" => self.handle_term(&mut results, token),
                "phrase" => self.handle_phrase(&mut results, token),
                _ => panic!("unexpected token type"),
            }
        }

        // Sort the results by rank (highest to lowest) then return them.
        results.sort_by(|a, b| b.cmp(a));
        results
    }

    /// Parse the incoming query string.
    ///
    /// Returns a map of query token types (eg. "term" or "phrase") to a vector
    /// of tokens for that type.
    ///
    /// For example, if the query is `steve "the hair" hairington`, then the
    /// search results will contain only documents that contain:
    /// * the term "steve"
    /// * the phrase "the hair"
    /// * the term "hairington"
    ///
    /// A phrase is matched based on the positions of each term in the phrase.
    /// In the above example, this method would return:
    /// ```
    /// [("term", "steve"), ("phrase", "the hair"), ("term", "hairington")]
    /// ```
    ///
    /// If the stop words filter is enabled, then stop words will be removed
    /// from the query.
    fn parse_query(&self, query: &str) -> Vec<(&str, String)> {
        let mut tokens = Vec::new();
        let mut token_type = "term";
        let mut token = String::new();
        let mut query = query.trim().to_lowercase();
        if self.stop_words {
            query = self.remove_stop_words(&query);
        }
        for c in query.chars() {
            if c == '"' {
                // Toggle between "term" and "phrase" tokens.
                if token_type == "term" {
                    token_type = "phrase";
                } else {
                    // Add the phrase to the list of tokens.
                    tokens.push((token_type, token.clone()));
                    token.clear();

                    token_type = "term";
                }
            } else if c == ' ' && token_type == "term" {
                if token.len() > 0 {
                    // Add the term to the list of tokens.
                    tokens.push((token_type, token.clone()));
                    token.clear();
                }
            } else {
                // Add the current character to the current token.
                token.push(c);
            }
        }
        // If there's a last token, add it to the token vector.
        if token.len() > 0 {
            tokens.push((token_type, token.clone()));
        }
        tokens
    }

    /// Removes stop words from the query.
    fn remove_stop_words(&self, query: &str) -> String {
        let mut new_query = String::new();
        let mut words = query.split_whitespace();
        while let Some(word) = words.next() {
            if !STOP_WORDS.contains(&word) {
                new_query.push_str(word);
                new_query.push(' ');
            }
        }
        new_query.pop(); // Removes the trailing space.
        new_query
    }

    /// Handles the first phrase in the query.
    ///
    /// For example, the phrase "the hair" will match documents that contain
    /// the term "the" followed by the term "hair". This is done by checking the
    /// positions of each term in the phrase. If the position of the term "the"
    /// was 0, and the position of the term "hair" was 4, then the document
    /// would match the phrase (the offset of the first term plus the length of
    /// the term, plus a space equals four).
    ///
    /// If the phrase is not matched, then the results are filtered out.
    fn handle_leading_phrase(&self, results: &mut Vec<QueryResult>, phrase: &str) {
        let mut terms = phrase.split_whitespace().into_iter();
        let first_term = terms.next().expect("phrase is empty");

        // Iterate over all the documents that contain the first term.
        if let Some(search_results) = self.mem_index.search(first_term) {
            for (doc_id, doc_positions) in search_results {
                // Iterate over the positions of the first term in this document
                let mut pos_iter = doc_positions.iter();
                let mut rank = doc_positions.len();
                'mid: while let Some(mut pos) = pos_iter.next().map(|off| *off) {
                    let mut prev_term = first_term;
                    for next_term in terms.by_ref() {
                        // Get the positions of the next term in this document.
                        if let Some(sr) = self.mem_index.search(next_term) {
                            if let Some(dp) = sr.get(&doc_id) {
                                let offset = pos + prev_term.len() + 1;
                                if dp.contains(&offset) {
                                    rank += dp.len();
                                    pos = offset;
                                    prev_term = next_term;
                                    continue;
                                }
                            }
                        }
                        break 'mid;
                    }
                    // We matched all the terms in the phrase!
                    let maybe_name = self.doc_table.get_name(*doc_id);
                    let name = maybe_name.expect("doc_id not found").to_string();
                    let qr = QueryResult::new(*doc_id, name, rank);
                    results.push(qr);
                    return;
                }
            }
        }
    }

    /// Handles the first term in the query.
    ///
    /// By definition, all the documents that match the term will be added
    /// to the results.
    fn handle_leading_term(&self, results: &mut Vec<QueryResult>, term: &str) {
        if let Some(search_results) = self.mem_index.search(term) {
            for (doc_id, positions) in search_results {
                let maybe_name = self.doc_table.get_name(*doc_id);
                let name = maybe_name.expect("doc_id not found").to_string();
                let roodrank = positions.len();
                let qr = QueryResult::new(*doc_id, name, roodrank);
                results.push(qr);
            }
        }
    }

    fn handle_term(&self, results: &mut Vec<QueryResult>, term: &str) {
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
                    false // Doesn't match, so remove it.
                }
            });
        } else {
            // No results for term, the entire query fails.
            results.clear();
        }
    }

    fn handle_phrase(&self, results: &mut Vec<QueryResult>, phrase: &str) {
        let mut terms = phrase.split_whitespace().into_iter();
        let first_term = terms.next().expect("phrase is empty");

        // Iterate over all the documents that contain the first term.
        if let Some(new_results) = self.mem_index.search(first_term) {
            // Go through each result we have so far and filter out any
            // results that don't match the current term.
            results.retain_mut(|qr| {
                if let Some(doc_positions) = new_results.get(&qr.doc_id) {
                    // Iterate over the positions of the first term in this document
                    let mut pos_iter = doc_positions.iter();
                    let mut rank = doc_positions.len();
                    'outer: while let Some(mut pos) = pos_iter.next().map(|off| *off) {
                        let mut prev_term = first_term;
                        for next_term in terms.by_ref() {
                            // Get the positions of the next term in this document.
                            if let Some(sr) = self.mem_index.search(next_term) {
                                if let Some(dp) = sr.get(&qr.doc_id) {
                                    let offset = pos + prev_term.len() + 1;
                                    if dp.contains(&offset) {
                                        rank += dp.len();
                                        pos = offset;
                                        prev_term = next_term;
                                        continue;
                                    }
                                }
                            }
                            break 'outer;
                        }

                        // We matched all the terms in the phrase!
                        qr.rank += rank;
                        return true;
                    }
                    false
                } else {
                    false // Doesn't match, so remove it.
                }
            });
        } else {
            // No results for term, the entire query fails.
            results.clear();
        }
    }
}
