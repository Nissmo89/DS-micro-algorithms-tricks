/// Reverses the order of bits in a `u32` integer.
///
/// This function performs a bitwise reversal of the input integer.
/// For example, if the input is `0b00000000000000000000000000001101` (13),
/// the output will be `0b10110000000000000000000000000000` (2952790016).
///
/// The reversal is achieved by a series of bit shifts and masks, effectively
/// swapping pairs of bits at increasing distances (1, 2, 4, 8, 16 bits).
///
/// # Arguments
///
/// * `n` - The `u32` integer whose bits are to be reversed.
///
/// # Returns
///
/// The `u32` integer with its bits reversed.
pub fn reverse_bits(mut n: u32) -> u32 {
    // Swap adjacent bits
    n = ((n >> 1) & 0x55555555) | ((n & 0x55555555) << 1);
    // Swap pairs of bits
    n = ((n >> 2) & 0x33333333) | ((n & 0x33333333) << 2);
    // Swap nibbles (4 bits)
    n = ((n >> 4) & 0x0F0F0F0F) | ((n & 0x0F0F0F0F) << 4);
    // Swap bytes (8 bits)
    n = ((n >> 8) & 0x00FF00FF) | ((n & 0x00FF00FF) << 8);
    // Swap 16-bit halves
    n = (n >> 16) | (n << 16);
    n
    // Note: Rust has a built-in `n.reverse_bits()` method, but this implementation
    // demonstrates the underlying bit manipulation technique.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_bits_zero() {
        assert_eq!(reverse_bits(0), 0);
    }

    #[test]
    fn test_reverse_bits_one() {
        // 1 is 0...001, reversed is 100...0 (which is 2^31)
        assert_eq!(reverse_bits(1), 1 << 31);
    }

    #[test]
    fn test_reverse_bits_max() {
        // u32::MAX is all 1s, reversing it results in all 1s.
        assert_eq!(reverse_bits(u32::MAX), u32::MAX);
    }

    #[test]
    fn test_reverse_bits_example() {
        // Input: 13 (0b0...01101)
        // Expected: 2952790016 (0b10110...0)
        assert_eq!(reverse_bits(13), 2952790016);
    }

    #[test]
    fn test_reverse_bits_specific_pattern() {
        // Input: 0b11110000101010100000111100001111
        let input: u32 = 0xF0A00F0F;
        // Expected: 0b11110000111100001010101000001111
        let expected: u32 = 0xF00F0A0F;
        assert_eq!(reverse_bits(input), expected);
    }
}