extern crate inverted_index;

#[test]
fn test_index_document() {
    use inverted_index::InvertedIndex;

    let mut idx = InvertedIndex::new();
    idx.index("Hello, World!".to_string());
}
