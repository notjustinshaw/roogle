use std::fs;
use std::io::Result;

use super::crawler::Crawler;
use crate::search_engine::indexer::doc_index::DocIndex;
use crate::search_engine::indexer::doc_table::DocTable;
use crate::search_engine::indexer::mem_index::MemIndex;

/// Crawls a filesystem and parses all files into an inverted index.
pub struct FileSystemCrawler {
    /// The root directory to crawl.
    root: String,

    /// Whether to index stop words.
    stop_words: bool,
}

impl FileSystemCrawler {
    /// Creates a new FileSystemCrawler.
    ///
    /// The root directory is the directory to crawl.
    pub fn new(root: &str, stop_words: bool) -> Self {
        Self {
            root: root.to_string(),
            stop_words,
        }
    }

    /// Crawls the root directory and returns a list of files.
    ///
    /// The list of files is returned as a vector of strings. Each string is the
    /// path to a file in the root directory.
    fn files(&self) -> Result<Vec<String>> {
        let mut files: Vec<String> = Vec::new();
        let mut stack: Vec<String> = Vec::new();
        stack.push(self.root.to_string());
        while let Some(dir) = stack.pop() {
            let mut entries = fs::read_dir(dir)?;
            while let Some(entry) = entries.next() {
                let path = entry?.path();
                let path_string: String = path.to_str().unwrap().to_string();
                if path.is_dir() {
                    stack.push(path_string);
                } else {
                    files.push(path_string);
                }
            }
        }
        Ok(files)
    }
}

impl Crawler for FileSystemCrawler {
    fn crawl(&self) -> Result<(DocTable, MemIndex)> {
        let mut doc_table = DocTable::new();
        let mut mem_index = MemIndex::new();
        for file_name in self.files()?.iter() {
            let doc_index: DocIndex = DocIndex::from_file(file_name, self.stop_words)?;
            let doc_id: usize = doc_table.add(file_name);
            mem_index.add(doc_index, doc_id);
        }
        Ok((doc_table, mem_index))
    }
}
