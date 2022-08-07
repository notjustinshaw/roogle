use std::fs;
use std::io::Result;

use super::crawler::Crawler;
use crate::indexer::doc_index::DocIndex;
use crate::indexer::doc_table::DocTable;
use crate::indexer::mem_index::MemIndex;

/// Crawls a filesystem and parses all files into an inverted index.
pub struct FileSystemCrawler {
    /// The root directory to crawl.
    pub(crate) root: String,
}

impl FileSystemCrawler {
    /// Creates a new FileSystemCrawler.
    ///
    /// The root directory is the directory to crawl.
    pub fn new(root: &str) -> Self {
        Self {
            root: root.to_string(),
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
                let entry = entry?; // Unwrap the entry.
                let path = entry.path();
                if path.is_dir() {
                    stack.push(path.to_str().unwrap().to_string());
                } else {
                    files.push(path.to_str().unwrap().to_string());
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
        for file in self.files()?.iter() {
            let doc_index: DocIndex = DocIndex::from_file(file)?;
            let doc_id: usize = doc_table.add(file);
            mem_index.add(doc_index, doc_id);
        }
        Ok((doc_table, mem_index))
    }
}
