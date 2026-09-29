pub fn longest_palindrome(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }
    // Transform string: ^#a#b#...#$
    let mut t: Vec<char> = Vec::with_capacity(s.len() * 2 + 3);
    t.push('^');
    for ch in s.chars() {
        t.push('#');
        t.push(ch);
    }
    t.push('#');
    t.push('$');
    let n = t.len();
    let mut p = vec![0usize; n];
    let mut center = 0usize;
    let mut right = 0usize;
    let mut max_len = 0usize;
    let mut max_center = 0usize;
    for i in 1..n-1 {
        let mirror = 2 * center - i;
        if i < right {
            p[i] = std::cmp::min(right - i, p[mirror]);
        }
        while t[i + 1 + p[i]] == t[i - 1 - p[i]] {
            p[i] += 1;
        }
        if i + p[i] > right {
            center = i;
            right = i + p[i];
        }
        if p[i] > max_len {
            max_len = p[i];
            max_center = i;
        }
    }
    let start = (max_center - max_len) / 2;
    s.chars().skip(start).take(max_len).collect()
}