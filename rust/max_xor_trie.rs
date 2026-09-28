use std::io::{self, Read};

struct TrieNode {
    children: [Option<Box<TrieNode>>; 2],
}

impl TrieNode {
    fn new() -> Self {
        TrieNode { children: [None, None] }
    }

    fn insert(&mut self, num: u32) {
        let mut node = self;
        for i in (0..32).rev() {
            let bit = ((num >> i) & 1) as usize;
            if node.children[bit].is_none() {
                node.children[bit] = Some(Box::new(TrieNode::new()));
            }
            node = node.children[bit].as_mut().unwrap();
        }
    }

    fn query(&self, num: u32) -> u32 {
        let mut node = self;
        let mut xor = 0u32;
        for i in (0..32).rev() {
            let bit = ((num >> i) & 1) as usize;
            let toggled = 1 - bit;
            if let Some(ref child) = node.children[toggled] {
                xor |= 1 << i;
                node = child.as_ref();
            } else if let Some(ref child) = node.children[bit] {
                node = child.as_ref();
            } else {
                break;
            }
        }
        xor
    }
}

fn max_xor_pair(nums: &[u32]) -> u32 {
    let mut trie = TrieNode::new();
    let mut max_xor = 0;
    for &num in nums {
        trie.insert(num);
        let current = trie.query(num);
        if current > max_xor {
            max_xor = current;
        }
    }
    max_xor
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let nums: Vec<u32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let result = max_xor_pair(&nums);
    println!("{}", result);
}