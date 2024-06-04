use std::collections::HashMap;

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
