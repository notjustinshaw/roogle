use crate::indexer::doc_table::DocTable;
use crate::indexer::mem_index::MemIndex;
use std::io::Result;

/// Crawl over some resource (eg. the filesystem) and build an index of all
/// documents found throughout that resource.
pub trait Crawler {
    /// Crawls a set of documents and parses them into an inverted index.
    fn crawl(&self) -> Result<(DocTable, MemIndex)>;
}
