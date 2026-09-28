pub fn length_of_lis(nums: &[i32]) -> usize {
    let mut tails: Vec<i32> = Vec::new();
    for &x in nums {
        match tails.binary_search(&x) {
            Ok(idx) => tails[idx] = x,
            Err(idx) => {
                if idx == tails.len() {
                    tails.push(x);
                } else {
                    tails[idx] = x;
                }
            }
        }
    }
    tails.len()
}

fn main() {
    let arr = [10, 9, 2, 5, 3, 7, 101, 18];
    println!("Length of LIS: {}", length_of_lis(&arr));
}