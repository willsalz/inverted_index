extern crate inverted_index;

use inverted_index::InvertedIndex;

fn main() {
    // Create and populate documents
    let mut idx = InvertedIndex::new();
    idx.index("The quick brown fox jumps over the lazy dog".to_string());
    idx.index("life, universe, and everything".to_string());
    idx.index("The world is flat".to_string());

    // Search for a term
    let query = "what the";
    let doc_ids = idx.search(query.to_string());

    // Display found documents
    for doc_id in &doc_ids {
        let doc = idx.lookup(*doc_id);
        println!("Query({}) -> DocId({}) -> Document({:?})",
                 query,
                 doc_id,
                 doc);
    }
    idx.delete(0);

    // Display found documents
    let doc_ids = idx.search(query.to_string());
    for doc_id in &doc_ids {
        let doc = idx.lookup(*doc_id);
        println!("Query({}) -> DocId({}) -> Document({:?})",
                 query,
                 doc_id,
                 doc);
    }
}
