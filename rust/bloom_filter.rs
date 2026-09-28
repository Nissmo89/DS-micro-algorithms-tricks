use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub struct BloomFilter {
    bits: Vec<u64>,
    size: usize,
}

impl BloomFilter {
    pub fn new(size: usize) -> Self {
        let words = (size + 63) / 64;
        BloomFilter { bits: vec![0; words], size }
    }

    fn hash1<T: Hash>(value: &T) -> u64 {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    }

    fn hash2<T: Hash>(value: &T) -> u64 {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish() ^ 0x9e3779b97f4a7c15
    }

    pub fn add<T: Hash>(&mut self, value: &T) {
        let h1 = Self::hash1(value) % self.size as u64;
        let h2 = Self::hash2(value) % self.size as u64;
        self.set_bit(h1 as usize);
        self.set_bit(h2 as usize);
    }

    pub fn contains<T: Hash>(&self, value: &T) -> bool {
        let h1 = Self::hash1(value) % self.size as u64;
        let h2 = Self::hash2(value) % self.size as u64;
        self.get_bit(h1 as usize) && self.get_bit(h2 as usize)
    }

    fn set_bit(&mut self, idx: usize) {
        let word = idx / 64;
        let bit = idx % 64;
        self.bits[word] |= 1u64 << bit;
    }

    fn get_bit(&self, idx: usize) -> bool {
        let word = idx / 64;
        let bit = idx % 64;
        (self.bits[word] >> bit) & 1u64 == 1
    }
}
