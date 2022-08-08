use simple_logger;
use std::io;
use std::time::Instant;

use clap::Parser;
use roogle::cli::CLI;
use roogle::search_engine::query_processor::query_processor::QueryProcessor;

fn main() {
    simple_logger::init().unwrap();

    println!("Welcome to Roogle!");
    let args = CLI::parse();

    eprint!("Indexing documents... ");
    let start_time = Instant::now();
    let qp = QueryProcessor::new("./assets/tiny", args.stop_words);
    let elapsed_sec = start_time.elapsed().as_secs_f64();
    println!("done!");

    println!(
        "Indexed {} documents ({} terms) in {:.2} seconds",
        qp.num_docs(),
        qp.num_terms(),
        elapsed_sec
    );
    println!();

    loop {
        process_query(&qp);
    }
}

fn process_query(qp: &QueryProcessor) {
    eprint!("Enter a query: ");
    let mut buf = String::new();
    match io::stdin().read_line(&mut buf) {
        Err(e) => println!("error: {}", e),
        Ok(_bytes_read) => {
            let start_time = Instant::now();
            let results = qp.search(&buf);
            let elapsed_us = start_time.elapsed().as_micros();
            match results.len() {
                0 => println!("No results found."),
                _ => {
                    let num = results.len();
                    for qr in results {
                        println!("  {} ({})", qr.doc_name, qr.rank);
                    }
                    println!("Found {} results in {:.2} us", num, elapsed_us);
                }
            }
        }
    }
    println!();
}
