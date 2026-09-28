pub fn z_algorithm(s: &str) -> Vec<usize> {
    let n = s.len();
    let bytes = s.as_bytes();
    let mut z = vec![0; n];
    let mut l = 0;
    let mut r = 0;
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

fn main() {
    let s = "ababcababa";
    let z = z_algorithm(s);
    println!("String: {}", s);
    println!("Z-array: {:?}", z);
}
