use super::Solve;
use std::collections::HashMap;

pub struct LongestCommonPrefix;

pub struct Node {
    children: HashMap<char, Node>,
    is_end_of_word: bool,
}

impl Node {
    pub fn new() -> Self {
        Node {
            children: HashMap::new(),
            is_end_of_word: false,
        }
    }

    pub fn insert(&mut self, string: String) {
        let mut current_node = self;
        for ch in string.chars() {
            current_node = current_node
                .children
                .entry(ch)
                .or_insert_with(|| Node::new());
        }
        current_node.is_end_of_word = true;
    }

    pub fn find_longest_common_prefix(&self) -> String {
        let mut result = String::new();
        let mut current_node = self;

        while current_node.children.len() == 1
            && (result.is_empty() || !current_node.is_end_of_word)
        {
            let (ch, child_node) = current_node.children.iter().next().unwrap();
            result.push(*ch);
            current_node = child_node;
        }

        result
    }
}

impl LongestCommonPrefix {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::new();
        }
        if strs.len() == 1 {
            return strs[0].clone();
        }

        let mut trie = Node::new();

        for string in strs {
            if string.is_empty() {
                return String::new();
            }

            trie.insert(string);
        }

        trie.find_longest_common_prefix()
    }
}

impl Solve<Vec<String>, String> for LongestCommonPrefix {
    fn solve(input: Vec<String>) -> String {
        Self::longest_common_prefix(input)
    }
}
