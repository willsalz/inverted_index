use std::collections::HashMap;

struct InvertedIndex {
    index: HashMap<String, Vec<String>>,
    documents: Vec<String>,
}

impl InvertedIndex {
    fn new() -> InvertedIndex {
        return InvertedIndex {
            index: HashMap::new(),
            documents: Vec::new(),
        };
    }

    fn tokenize(&self, document: String) -> Vec<String> {
        let tokens: Vec<String> = vec![document, "wut".to_owned()];
        return tokens;
    }

    fn index(&self, document: String) -> i32 {
        // self.documents.push(document);
        let tokens = self.tokenize(document);
        for token in tokens {
            println!("{}", token);
        }
        0
    }

    fn search(&self, query: String) -> i32 {
        0
    }
}


fn main() {
    let idx = InvertedIndex::new();
    idx.index("The quick brown fox jumps over the lazy dog".to_owned());
    println!("{}", idx.search("Hello, World?".to_owned()));
}
