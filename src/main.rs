extern crate rand;

use std::collections::{HashMap, HashSet};

use rand::random;

// Documents
type DocId = u64;

// Index
struct InvertedIndex<'x> {
    // Token --> [ DocId ]
    index: HashMap<&'x str, Vec<DocId>>,
}

impl<'x> InvertedIndex<'x> {
    fn new() -> InvertedIndex<'x> {
        return InvertedIndex { index: HashMap::new() };
    }

    fn tokenize(&self, payload: &'x str) -> Vec<&'x str> {
        return payload.split_whitespace().collect();
    }

    fn index(&mut self, payload: &'x str) -> DocId {
        let id: DocId = random();
        let tokens = self.tokenize(payload);
        for token in tokens {
            self.index.entry(token).or_insert(vec![]).push(id);
        }
        id
    }

    fn search(&self, query: &str) -> HashSet<DocId> {
        let mut docs: HashSet<DocId> = HashSet::new();
        let tokens = self.tokenize(query);
        for token in tokens {
            match self.index.get(token) {
                Some(ids) => {
                    for id in ids {
                        docs.insert(*id);
                    }
                }
                None => {}
            }
        }
        docs
    }
}


fn main() {
    let mut idx = InvertedIndex::new();
    idx.index("The quick brown fox jumps over the lazy dog");
    idx.index("the world is flat");
    println!("{:?}", idx.search("the"));
    println!("{:?}", idx.search("foo"));
}
