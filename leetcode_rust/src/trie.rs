use std::{collections::HashMap, vec};

#[derive(Default)]
struct Trie {
    children: HashMap<char, Trie>,
    is_leaf: bool,
}

#[allow(dead_code)]
impl Trie {

    fn new() -> Self {
        Trie::default()
    }
    
    fn insert(&mut self, word: String) {
        word.chars()
        .fold(self, |node, c| node.children.entry(c).or_default())
        .is_leaf = true;
    }
    
    fn search(&self, word: String) -> bool {
        return self.get(word).map_or(false, |n| n.is_leaf);
    }
    
    fn starts_with(&self, prefix: String) -> bool {
        return self.get(prefix).is_some();
    }

    fn get(&self, word: String) -> Option<&Trie> {
        return word.chars().try_fold(self, |node, c| node.children.get(&c));
    }
}

#[allow(dead_code)]
pub fn suggested_products(mut products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
    products.sort_unstable();

    let mut result = vec![];
    let mut prefix = String::new();

    for c in search_word.chars() {
        prefix.push(c);

        let index = match products.binary_search(&prefix) {
            Ok(i) => i,
            Err(i) => i,
        };

        result.push(
            products[index..]
            .iter()
            .take(3)
            .take_while(|p| p.starts_with(&prefix))
            .cloned()
            .collect(),
        );
    }

    result
}