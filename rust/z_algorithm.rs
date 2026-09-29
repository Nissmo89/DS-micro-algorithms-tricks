pub fn z_algorithm(s: &str) -> Vec<usize> {
    let bytes = s.as_bytes();
    let n = bytes.len();
    let mut z = vec![0; n];
    let mut l = 0usize;
    let mut r = 0usize;
    for i in 1..n {
        if i <= r {
            z[i] = std::cmp::min(r - i + 1, z[i - l]);
        }
        while i + z[i] < n && bytes[z[i]] == bytes[i + z[i]] {
            z[i] += 1;
        }
        if i + z[i] - 1 > r {
            l = i;
            r = i + z[i] - 1;
        }
    }
    z
}

pub fn find_occurrences(pattern: &str, text: &str) -> Vec<usize> {
    if pattern.is_empty() {
        return Vec::new();
    }
    let concat = format!("{}#{}", pattern, text);
    let z = z_algorithm(&concat);
    let m = pattern.len();
    let mut res = Vec::new();
    for i in m + 1..z.len() {
        if z[i] == m {
            res.push(i - m - 1);
        }
    }
    res
}
