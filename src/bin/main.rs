use std::io;

use roogle::query_processor::query_processor::QueryProcessor;

fn main() {
    println!("Welcome to Roogle!");
    eprint!("Indexing documents... ");
    let qp = QueryProcessor::new("./assets/");
    println!("done!");
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
            let results = qp.search(&buf);
            match results.len() {
                0 => println!("No results found."),
                _ => {
                    println!("Found {} results:", results.len());
                    for qr in results {
                        println!("  {} ({})", qr.doc_name, qr.rank);
                    }
                }
            }
        }
    }
    println!();
}
