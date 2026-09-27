pub fn lis_length<T: Ord + Clone>(arr: &[T]) -> usize {
    let mut tails: Vec<T> = Vec::new();
    for x in arr {
        match tails.binary_search(x) {
            Ok(idx) | Err(idx) => {
                if idx == tails.len() {
                    tails.push(x.clone());
                } else {
                    tails[idx] = x.clone();
                }
            }
        }
    }
    tails.len()
}
