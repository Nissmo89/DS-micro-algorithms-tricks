use std::io;

fn lower_bound(arr: &Vec<i32>, target: i32) -> usize {
    let mut left = 0;
    let mut right = arr.len();
    while left < right {
        let mid = left + (right - left) / 2;
        if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left
}

fn lis_length(nums: Vec<i32>) -> usize {
    let mut tails: Vec<i32> = Vec::new();
    for &x in nums.iter() {
        let idx = lower_bound(&tails, x);
        if idx == tails.len() {
            tails.push(x);
        } else {
            tails[idx] = x;
        }
    }
    tails.len()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let len = lis_length(nums);
    println!("{}", len);
}
