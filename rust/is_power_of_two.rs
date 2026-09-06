/// Checks if a given unsigned 64-bit integer is a power of two.
///
/// A number `n` is a power of two if and only if it is positive and has exactly one bit set
/// in its binary representation. This can be efficiently checked using the expression
/// `n > 0 && (n & (n - 1)) == 0`.
///
/// Explanation:
/// - If `n` is a power of two (e.g., 8, binary `1000`), then `n - 1` will have all bits
///   to the right of the set bit as 1 (e.g., 7, binary `0111`).
/// - Performing a bitwise AND (`&`) between `n` and `n - 1` will result in 0
///   (`1000 & 0111 = 0000`).
/// - If `n` is not a power of two (e.g., 6, binary `0110`), it has multiple set bits.
///   `n - 1` (e.g., 5, binary `0101`).
/// - `n & (n - 1)` will not be 0 (`0110 & 0101 = 0100`).
/// - The `n > 0` check handles the case where `n` is 0, which is not considered a power of two.
///
/// # Arguments
///
/// * `n` - The `u64` integer to check.
///
/// # Returns
///
/// `true` if `n` is a power of two, `false` otherwise.
pub fn is_power_of_two(n: u64) -> bool {
    n > 0 && (n & (n - 1)) == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_power_of_two_true() {
        assert!(is_power_of_two(1));      // 2^0
        assert!(is_power_of_two(2));      // 2^1
        assert!(is_power_of_two(4));      // 2^2
        assert!(is_power_of_two(16));     // 2^4
        assert!(is_power_of_two(1024));   // 2^10
        assert!(is_power_of_two(1 << 30)); // 2^30
        assert!(is_power_of_two(1 << 60)); // 2^60
    }

    #[test]
    fn test_is_power_of_two_false() {
        assert!(!is_power_of_two(0));
        assert!(!is_power_of_two(3));
        assert!(!is_power_of_two(5));
        assert!(!is_power_of_two(6));
        assert!(!is_power_of_two(10));
        assert!(!is_power_of_two(100));
        assert!(!is_power_of_two(u64::MAX));
    }
}