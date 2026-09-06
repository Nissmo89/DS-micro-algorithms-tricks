/// Calculates the integer logarithm base 2 of a `u32` integer.
///
/// This function returns the largest integer `k` such that `2^k <= n`.
/// It utilizes bit manipulation to efficiently find the position of the most significant bit (MSB).
/// The position of the MSB is the integer log base 2.
///
/// For `n = 0`, the behavior is undefined or could be considered negative infinity.
/// This implementation returns 0 for `n = 0` for simplicity, though a more robust
/// version might panic or return an Option/Result.
///
/// # Arguments
///
/// * `n` - The input `u32` integer.
///
/// # Returns
///
/// The integer logarithm base 2 of `n`.
pub fn integer_log2(mut n: u32) -> u32 {
    if n == 0 {
        return 0; // Or panic!, depending on desired behavior for log(0)
    }

    let mut log = 0;
    // Repeatedly divide n by 2 (right shift) until it becomes 1.
    // The number of shifts is the integer log base 2.
    while n > 1 {
        n >>= 1; // Equivalent to n = n / 2
        log += 1;
    }
    log

    // A more optimized version using bitwise tricks (like counting leading zeros):
    // if n == 0 { return 0; } // Or panic
    // 31 - n.leading_zeros()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log2_zero() {
        assert_eq!(integer_log2(0), 0);
    }

    #[test]
    fn test_log2_one() {
        assert_eq!(integer_log2(1), 0);
    }

    #[test]
    fn test_log2_powers_of_two() {
        assert_eq!(integer_log2(2), 1);
        assert_eq!(integer_log2(4), 2);
        assert_eq!(integer_log2(8), 3);
        assert_eq!(integer_log2(16), 4);
        assert_eq!(integer_log2(1024), 10);
        assert_eq!(integer_log2(1 << 30), 30);
    }

    #[test]
    fn test_log2_non_powers_of_two() {
        assert_eq!(integer_log2(3), 1);
        assert_eq!(integer_log2(5), 2);
        assert_eq!(integer_log2(7), 2);
        assert_eq!(integer_log2(15), 3);
        assert_eq!(integer_log2(1000), 9); // 2^9 = 512, 2^10 = 1024
    }

    #[test]
    fn test_log2_max_value() {
        // For u32::MAX, the MSB is at position 31.
        assert_eq!(integer_log2(u32::MAX), 31);
    }
}