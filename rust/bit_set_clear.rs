/// Sets a specific bit in an integer to 1.
///
/// # Arguments
///
/// * `n` - The integer whose bit is to be set.
/// * `pos` - The position of the bit to set (0-indexed from the right).
///
/// # Returns
///
/// The integer with the bit at `pos` set to 1.
pub fn set_bit(n: u32, pos: u8) -> u32 {
    n | (1 << pos)
}

/// Clears a specific bit in an integer to 0.
///
/// # Arguments
///
/// * `n` - The integer whose bit is to be cleared.
/// * `pos` - The position of the bit to clear (0-indexed from the right).
///
/// # Returns
///
/// The integer with the bit at `pos` cleared to 0.
pub fn clear_bit(n: u32, pos: u8) -> u32 {
    n & !(1 << pos)
}

/// Toggles (flips) a specific bit in an integer.
///
/// # Arguments
///
/// * `n` - The integer whose bit is to be toggled.
/// * `pos` - The position of the bit to toggle (0-indexed from the right).
///
/// # Returns
///
/// The integer with the bit at `pos` toggled.
pub fn toggle_bit(n: u32, pos: u8) -> u32 {
    n ^ (1 << pos)
}

/// Checks if a specific bit is set (1).
///
/// # Arguments
///
/// * `n` - The integer to check.
/// * `pos` - The position of the bit to check (0-indexed from the right).
///
/// # Returns
///
/// `true` if the bit at `pos` is set, `false` otherwise.
pub fn is_bit_set(n: u32, pos: u8) -> bool {
    (n & (1 << pos)) != 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_bit() {
        let n = 0b1010; // 10
        assert_eq!(set_bit(n, 0), 0b1011); // 11
        assert_eq!(set_bit(n, 1), 0b1010); // 10 (bit already set)
        assert_eq!(set_bit(n, 2), 0b1110); // 14
    }

    #[test]
    fn test_clear_bit() {
        let n = 0b1011; // 11
        assert_eq!(clear_bit(n, 0), 0b1010); // 10
        assert_eq!(clear_bit(n, 1), 0b1011); // 11 (bit already clear)
        assert_eq!(clear_bit(n, 3), 0b0011); // 3
    }

    #[test]
    fn test_toggle_bit() {
        let n = 0b1010; // 10
        assert_eq!(toggle_bit(n, 0), 0b1011); // 11 (clear to set)
        assert_eq!(toggle_bit(n, 1), 0b1000); // 8  (set to clear)
        assert_eq!(toggle_bit(n, 2), 0b1110); // 14 (clear to set)
    }

    #[test]
    fn test_is_bit_set() {
        let n = 0b1010; // 10
        assert_eq!(is_bit_set(n, 0), false);
        assert_eq!(is_bit_set(n, 1), true);
        assert_eq!(is_bit_set(n, 2), false);
        assert_eq!(is_bit_set(n, 3), true);
    }
}