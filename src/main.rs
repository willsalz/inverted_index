extern crate rand;

use std::collections::{HashMap, HashSet};

use rand::random;

// Documents
type DocId = u64;

// Index
struct InvertedIndex {
    // Token --> [ DocId ]
    index: HashMap<String, Vec<DocId>>,

    // DocId --> Body
    docs: HashMap<DocId, String>,
}

impl InvertedIndex {
    fn new() -> InvertedIndex {
        return InvertedIndex {
            index: HashMap::new(),
            docs: HashMap::new(),
        };
    }

    fn tokenize(&self, payload: &str) -> Vec<String> {
        let lower = payload.to_lowercase();
        let tokens: Vec<_> = lower.split_whitespace().collect();
        let mut ret = Vec::new();
        for token in tokens {
            ret.push(token.to_string());
        }
        ret
    }

    fn index(&mut self, payload: String) -> DocId {
        // Generate document id
        let id: DocId = random();

        // Tokenize document
        let tokens = self.tokenize(&payload);

        // Store whole document
        self.docs.insert(id, payload);

        // Index document by tokens
        for token in tokens {
            self.index
                .entry(token)
                .or_insert(vec![])
                .push(id);
        }
        id
    }

    fn search(&self, query: String) -> HashSet<DocId> {
        let mut docs = HashSet::new();
        let tokens = self.tokenize(&query);
        for token in &tokens {
            match self.index.get(token) {
                Some(ids) => {
                    for id in ids {
                        docs.insert(*id);
                    }
                }
                _ => {}
            }
        }
        docs
    }

    fn lookup(&self, doc_id: DocId) -> Option<&String> {
        self.docs.get(&doc_id)
        // match self.docs.get(&doc_id) {
        //     Some(doc) => println!("{}: {}", doc_id, doc),
        //     None => println!("None"),
        // }
    }
}


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
}
