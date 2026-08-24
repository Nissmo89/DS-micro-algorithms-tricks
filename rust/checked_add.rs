/// Performs addition of two `u64` integers, returning `None` if overflow occurs.
///
/// This function uses Rust's built-in `checked_add` method, which is a
/// safe way to perform arithmetic operations that might exceed the maximum
/// value of the integer type. It avoids panics and provides a clear way
/// to handle potential overflow scenarios.
///
/// # Arguments
///
/// * `a` - The first operand.
/// * `b` - The second operand.
///
/// # Returns
///
/// An `Option<u64>`: `Some(result)` if the addition is successful without overflow,
/// or `None` if overflow occurs.
pub fn checked_add(a: u64, b: u64) -> Option<u64> {
    a.checked_add(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checked_add_no_overflow() {
        assert_eq!(checked_add(10, 20), Some(30));
        assert_eq!(checked_add(u64::MAX - 1, 1), Some(u64::MAX));
    }

    #[test]
    fn test_checked_add_overflow() {
        assert_eq!(checked_add(u64::MAX, 1), None);
        assert_eq!(checked_add(u64::MAX, u64::MAX), None);
    }

    #[test]
    fn test_checked_add_zero() {
        assert_eq!(checked_add(0, 0), Some(0));
        assert_eq!(checked_add(0, u64::MAX), Some(u64::MAX));
        assert_eq!(checked_add(u64::MAX, 0), Some(u64::MAX));
    }
}