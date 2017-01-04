use std::collections::{HashMap, HashSet};

// Documents
type DocId = u64;

// Index
#[derive(Debug)]
pub struct InvertedIndex {
    // Token --> [ DocId ]
    index: HashMap<String, Vec<DocId>>,

    // DocId --> Body
    docs: HashMap<DocId, String>,

    num_docs: u64,
}

impl InvertedIndex {
    pub fn new() -> InvertedIndex {
        return InvertedIndex {
            index: HashMap::new(),
            docs: HashMap::new(),
            num_docs: 0,
        };
    }

    fn tokenize(&self, payload: &str) -> Vec<String> {
        let lower = payload.to_lowercase();
        lower.split_whitespace().map(|token| token.to_string()).collect()
    }

    pub fn index(&mut self, payload: String) -> DocId {
        // Generate document id
        let id: DocId = self.num_docs;
        self.num_docs += 1;

        // Tokenize document
        let tokens = self.tokenize(&payload);

        // Store whole document
        self.docs.insert(id, payload);

        // Index document by tokens
        for token in tokens {
            self.index
                .entry(token)
                .or_insert(Vec::new())
                .push(id);
        }
        id
    }

    pub fn delete(&mut self, doc_id: DocId) -> () {
        // Remove DocId from each token's posting list
        for (_, mut posting_list) in &mut self.index {
            posting_list.retain(|val| *val != doc_id)
        }

        // Remove document
        self.docs.remove(&doc_id);

        // Decrement num_docs
        self.num_docs -= 1;
    }

    pub fn search(&self, query: String) -> HashSet<DocId> {
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

    pub fn lookup(&self, doc_id: DocId) -> Option<&String> {
        self.docs.get(&doc_id)
    }
}
