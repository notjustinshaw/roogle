use log::debug;

use crate::search_engine::filters::stop_words::STOP_WORDS;
use crate::search_engine::indexer::doc_table::DocTable;
use crate::search_engine::indexer::mem_index::MemIndex;
use std::fmt::Error;
use std::fmt::Formatter;
use std::fmt::{Debug, Display};

use super::query_result::QueryResult;

pub enum QueryToken {
    Term { value: String },
    Phrase { value: String },
}

impl QueryToken {
    pub fn push(&mut self, c: char) {
        match self {
            QueryToken::Term { value } => value.push(c),
            QueryToken::Phrase { value } => value.push(c),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            QueryToken::Term { value } => value.is_empty(),
            QueryToken::Phrase { value } => value.is_empty(),
        }
    }

    pub fn search(&self, index: &MemIndex, docs: &DocTable) -> Vec<QueryResult> {
        match self {
            QueryToken::Term { value } => handle_term(value, index, docs),
            QueryToken::Phrase { value } => handle_phrase(value, index, docs),
        }
    }
}

impl Display for QueryToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            QueryToken::Term { value } => write!(f, "Term({})", value),
            QueryToken::Phrase { value } => write!(f, "Phrase({})", value),
        }
    }
}

impl Debug for QueryToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            QueryToken::Term { value } => write!(f, "Term({})", value),
            QueryToken::Phrase { value } => write!(f, "Phrase({})", value),
        }
    }
}

impl PartialEq for QueryToken {
    fn eq(&self, other: &Self) -> bool {
        match self {
            QueryToken::Term { value: my_val } => match other {
                QueryToken::Term { value: other_val } => my_val == other_val,
                _ => false,
            },
            QueryToken::Phrase { value: my_val } => match other {
                QueryToken::Phrase { value: other_val } => my_val == other_val,
                _ => false,
            },
        }
    }
}

/// Parse the incoming query string and returns a vector of `QueryToken`.
///
/// For example, if the query is `steve "the hair" hairington`, then the
/// search results will contain only documents that contain:
///
/// ```txt
/// [
///   QueryToken::Term("steve"),
///   QueryToken::Phrase("the hair"),
///   QueryToken::Term("hairington")
/// ]
/// ```
///
/// If the stop words filter is enabled, then stop words will be removed
/// from the query.
///
/// # Examples
///
/// ```
/// use roogle::search_engine::query_processor::query_token::query_to_tokens;
/// use roogle::search_engine::query_processor::query_token::QueryToken;
///
/// let tokens = query_to_tokens("steve \"the hair\" hairington", false);
/// let mut iter = tokens.iter();
///
/// assert_eq!("Term(steve)".to_string(), iter.next().unwrap().to_string());
/// assert_eq!("Phrase(the hair)".to_string(), iter.next().unwrap().to_string());
/// assert_eq!("Term(hairington)".to_string(), iter.next().unwrap().to_string());
/// ```
pub fn query_to_tokens(query: &str, stop_words: bool) -> Vec<QueryToken> {
    let mut query: String = query.trim().to_lowercase();
    let mut tokens: Vec<QueryToken> = Vec::new();
    let mut token: QueryToken = QueryToken::Term {
        value: String::new(),
    };
    if stop_words {
        query = remove_stop_words(&query);
    }
    for c in query.chars() {
        if c == '"' {
            // Toggle between "term" and "phrase" tokens.
            match token {
                QueryToken::Term { value: _ } => {
                    token = QueryToken::Phrase {
                        value: String::new(),
                    };
                }
                QueryToken::Phrase { value: _ } => {
                    // Add the phrase to the list of tokens.
                    tokens.push(token);
                    token = QueryToken::Term {
                        value: String::new(),
                    };
                }
            }
        } else if c == ' ' {
            match token {
                QueryToken::Term {
                    value: ref mut term,
                } if term.len() > 0 => {
                    // Add the term to the list of tokens.
                    tokens.push(token);
                    token = QueryToken::Term {
                        value: String::new(),
                    };
                }
                QueryToken::Phrase { value: _ } => {
                    // Add the space to the phrase.
                    token.push(c);
                }
                _ => {}
            }
        } else {
            // Add the current character to the current token.
            token.push(c);
        }
    }
    // If there's a last token, add it to the token vector.
    if !token.is_empty() {
        tokens.push(token);
    }
    tokens
}

/// Removes stop words from the query.
fn remove_stop_words(query: &String) -> String {
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

/// Handles search for a single term.
fn handle_term(term: &str, index: &MemIndex, docs: &DocTable) -> Vec<QueryResult> {
    let mut query_results: Vec<QueryResult> = Vec::new();
    if let Some(results) = index.search(term) {
        results.iter().for_each(|(doc_id, postings)| {
            query_results.push(QueryResult::from(*doc_id, postings, docs));
        });
    }
    query_results
}

/// Handles search for a phrase.
fn handle_phrase(phrase: &str, index: &MemIndex, docs: &DocTable) -> Vec<QueryResult> {
    debug!("memindex: {}", index);

    let mut query_results: Vec<QueryResult> = Vec::new();
    let mut terms = phrase.split_whitespace().into_iter();
    let first_term = terms.next().unwrap();

    // Iterate over all the documents that contain the first term.
    debug!("looking for first term: {}", first_term);
    if let Some(search_results) = index.search(first_term) {
        'outer: for (doc_id, doc_positions) in search_results {
            // Iterate over the positions of the first term in this document
            debug!("  looking in document: {}", doc_id);
            let mut pos_iter = doc_positions.iter();
            let mut rank = doc_positions.len();
            let mut prev_term = first_term;
            'mid: while let Some(mut pos) = pos_iter.next().map(|off| *off) {
                debug!("    looking at position: {}", pos);

                // reset the iterator
                terms = phrase.split_whitespace().into_iter();
                terms.next(); // eat the first term

                for next_term in terms.by_ref() {
                    // Get the positions of the next term in this document.
                    debug!(
                        "      looking for '{}' followed by '{}'",
                        prev_term, next_term
                    );
                    if let Some(sr) = index.search(next_term) {
                        if let Some(dp) = sr.get(&doc_id) {
                            let offset = pos + prev_term.len() + 1;
                            debug!(
                                "      looking for '{}' starting at offset {}",
                                next_term, offset
                            );
                            if dp.contains(&offset) {
                                debug!("{} {} {}", doc_id, offset, next_term);
                                rank += dp.len();
                                pos = offset;
                                prev_term = next_term;
                                continue;
                            }
                            debug!("      offset {} not found in {:?}", offset, dp);
                            continue 'mid;
                        }
                        debug!("the term '{}' is not in document '{}'", next_term, doc_id);
                    }
                    // If we get here, then the phrase is not in this document.
                    debug!("no results for term '{}'", next_term);
                    continue 'mid;
                }
                // We matched all the terms in the phrase!
                debug!("yay");
                let maybe_name = docs.get_name(*doc_id);
                let name = maybe_name.expect("doc_id not found").to_string();
                let qr = QueryResult::new(*doc_id, name, rank);
                query_results.push(qr);
                continue 'outer;
            }
        }
    }
    query_results
}
