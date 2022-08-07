use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::{BufReader, Read, Result};
use std::mem;

/// An inverted index of word positions for a single document.
///
/// The inverted index is a mapping from terms to a list of positions in the
/// file where that term occurs. For example, the inverted index for the
/// file "My oh my!" might be:
///
/// ```text
/// {
///  "my": [0, 6],
///  "oh": [3],
/// }
/// ```
///
/// A term is a sequence of non-whitespace characters, with leading and
/// trailing punctuation removed. The position of each term is 0-based, so
/// "my" occurs at positions 0 and 6, and "oh" occurs at position 3. The
/// positions are byte offsets, not character or graphmeme offsets.
pub struct DocIndex {
    pub(crate) index: HashMap<String, Vec<usize>>,
    pub(crate) name: String,
}

impl DocIndex {
    /// Creates a new FileParser.
    ///
    /// The inverted index is initially empty. You must call `parse_file` to
    /// populate it.
    pub fn new(name: &str) -> Self {
        Self {
            index: HashMap::new(),
            name: name.to_string(),
        }
    }

    /// Returns the name of this document.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Parses a file into the inverted index.
    ///
    /// The inverted index is populated with the contents of the file. The
    /// file is read line by line, and each line is split into words. For each
    /// word, the inverted index is updated with the position of the word in the
    /// file.
    ///
    /// # Arguments
    /// * `filename` - The name of the file to parse.
    ///
    /// # Errors
    /// * If the file cannot be opened, then an error is returned.
    /// * If the file contains invalid bytes, then an error is returned.
    /// * If a term cannot be parsed, then an error is returned.
    pub fn from_file(filename: &str) -> Result<Self> {
        // Try to open the file for reading.
        let f = File::open(filename).expect("failed to open file");
        let reader = BufReader::new(f);
        let mut wordpos = Self::new(filename);

        // Read the file byte-by-byte and append contents into a word vector.
        let mut pos: usize = 0;
        let mut word: Vec<u8> = Vec::new();
        for maybe_byte in reader.bytes() {
            let mut byte = maybe_byte.expect("failed to read byte");

            // If the byte is *not* a whitespace character, then append it to
            // our vector of bytes (the "word vector").
            if !char::is_whitespace(byte as char) {
                byte.make_ascii_lowercase();
                word.push(byte);

            // If the byte is a whitespace character, then we *might* have
            // reached the end of a word (ie. the word vector is not empty)
            } else if word.len() > 0 {
                // To flush the word vector, replace it with an empty vector,
                // then parse the old vector into a String, trim punctuation,
                // and add it to the inverted index.
                let chars: Vec<u8> = mem::replace(&mut word, Vec::new());
                let raw_key = String::from_utf8_lossy(&chars);
                let is_punctuation = |c: char| c.is_ascii_punctuation();
                let key = raw_key.trim_matches(is_punctuation);
                // The start position of the word (protect against empty words)
                let start = usize::saturating_sub(pos, key.len());
                wordpos
                    .index
                    .entry(key.to_string())
                    .or_insert(Vec::new())
                    .push(start);
            }
            pos += 1;
        }
        Ok(wordpos)
    }
}

impl Display for DocIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:#?}", self.index)
    }
}

impl Debug for DocIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.index)
    }
}
