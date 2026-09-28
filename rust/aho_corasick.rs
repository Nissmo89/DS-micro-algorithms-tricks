use std::collections::VecDeque;

#[derive(Debug)]
struct Node {
    children: [Option<usize>; 26],
    fail: usize,
    output: Vec<usize>,
}

pub struct AhoCorasick {
    nodes: Vec<Node>,
    patterns: Vec<String>,
}

impl AhoCorasick {
    pub fn new() -> Self {
        Self {
            nodes: vec![Node {
                children: [None; 26],
                fail: 0,
                output: Vec::new(),
            }],
            patterns: Vec::new(),
        }
    }

    pub fn add_pattern(&mut self, pat: &str) {
        let mut node = 0;
        for ch in pat.chars() {
            let idx = (ch as u8 - b'a') as usize;
            let next = match self.nodes[node].children[idx] {
                Some(n) => n,
                None => {
                    let new_index = self.nodes.len();
                    self.nodes.push(Node {
                        children: [None; 26],
                        fail: 0,
                        output: Vec::new(),
                    });
                    self.nodes[node].children[idx] = Some(new_index);
                    new_index
                }
            };
            node = next;
        }
        self.nodes[node].output.push(self.patterns.len());
        self.patterns.push(pat.to_string());
    }

    pub fn build(&mut self) {
        let mut queue = VecDeque::new();
        // Initialize fail links of depth-1 nodes to root
        for idx in 0..26 {
            if let Some(child) = self.nodes[0].children[idx] {
                self.nodes[child].fail = 0;
                queue.push_back(child);
            }
        }
        while let Some(r) = queue.pop_front() {
            for idx in 0..26 {
                if let Some(s) = self.nodes[r].children[idx] {
                    let mut f = self.nodes[r].fail;
                    while f != 0 && self.nodes[f].children[idx].is_none() {
                        f = self.nodes[f].fail;
                    }
                    if let Some(&next) = self.nodes[f].children[idx] {
                        self.nodes[s].fail = next;
                    } else {
                        self.nodes[s].fail = 0;
                    }
                    let fail_output = self.nodes[self.nodes[s].fail].output.clone();
                    self.nodes[s].output.extend(fail_output);
                    queue.push_back(s);
                }
            }
        }
    }

    pub fn search(&self, text: &str) -> Vec<(usize, usize)> {
        let mut results = Vec::new();
        let mut state = 0;
        for (i, ch) in text.chars().enumerate() {
            let idx = (ch as u8 - b'a') as usize;
            while state != 0 && self.nodes[state].children[idx].is_none() {
                state = self.nodes[state].fail;
            }
            if let Some(&next) = self.nodes[state].children[idx] {
                state = next;
            }
            for &pat_idx in &self.nodes[state].output {
                results.push((pat_idx, i + 1 - self.patterns[pat_idx].len()));
            }
        }
        results
    }
}
