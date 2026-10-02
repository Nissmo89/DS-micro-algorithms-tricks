use std::cmp::max;

struct Trie {
    children: Vec<[i32; 2]>,
}

impl Trie {
    fn new() -> Self {
        Trie { children: vec![[-1, -1]] }
    }

    fn insert(&mut self, num: i32) {
        let mut node = 0;
        for i in (0..32).rev() {
            let bit = ((num >> i) & 1) as usize;
            if self.children[node][bit] == -1 {
                self.children[node][bit] = self.children.len() as i32;
                self.children.push([-1, -1]);
            }
            node = self.children[node][bit] as usize;
        }
    }

    fn query(&self, num: i32) -> i32 {
        let mut node = 0;
        let mut xor = 0;
        for i in (0..32).rev() {
            let bit = ((num >> i) & 1) as usize;
            let toggled = bit ^ 1;
            if self.children[node][toggled] != -1 {
                xor |= 1 << i;
                node = self.children[node][toggled] as usize;
            } else {
                node = self.children[node][bit] as usize;
            }
        }
        xor
    }
}

fn maximum_xor(nums: Vec<i32>) -> i32 {
    if nums.len() < 2 {
        return 0;
    }
    let mut trie = Trie::new();
    for &num in &nums {
        trie.insert(num);
    }
    let mut max_xor = 0;
    for &num in &nums {
        let cur = trie.query(num);
        if cur > max_xor {
            max_xor = cur;
        }
    }
    max_xor
}
