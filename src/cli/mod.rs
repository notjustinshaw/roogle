use clap::Parser;

/// A simple search engine written in Rust.
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct CLI {
    /// Whether to exclude stop words from the search (default: false).
    #[clap(short, long)]
    pub stop_words: bool,
}
