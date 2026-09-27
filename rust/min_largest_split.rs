fn can_split(nums: &[i32], m: usize, max_sum: i32) -> bool {
    let mut count = 1;
    let mut cur_sum = 0;
    for &x in nums {
        if cur_sum + x > max_sum {
            count += 1;
            cur_sum = x;
            if count > m { return false; }
        } else {
            cur_sum += x;
        }
    }
    true
}

fn split_array(nums: &[i32], m: usize) -> i32 {
    let mut low = *nums.iter().max().unwrap();
    let mut high: i32 = nums.iter().sum();
    while low < high {
        let mid = low + (high - low) / 2;
        if can_split(nums, m, mid) {
            high = mid;
        } else {
            low = mid + 1;
        }
    }
    low
}

fn main() {
    let nums = [7, 2, 5, 10, 8];
    let m = 2;
    println!("Minimum largest sum: {}", split_array(&nums, m));
}
