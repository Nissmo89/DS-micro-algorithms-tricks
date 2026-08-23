/// Calculates `base` raised to the power of `exp` using exponentiation by squaring.
///
/// This algorithm significantly reduces the number of multiplications required,
/// achieving O(log exp) time complexity.
///
/// # Arguments
///
/// * `base` - The base number.
/// * `exp` - The exponent (must be non-negative).
///
/// # Returns
///
/// The result of `base.pow(exp)`.
///
/// # Panics
///
/// Panics if `exp` is negative.
pub fn fast_power(mut base: u64, mut exp: u32) -> u64 {
    if exp == 0 {
        return 1;
    }
    if exp < 0 {
        panic!("Exponent must be non-negative");
    }

    let mut result = 1;

    // Exponentiation by squaring (binary exponentiation)
    // We iterate through the bits of the exponent.
    // If the current bit is 1, we multiply the result by the current base.
    // We then square the base for the next bit.
    while exp > 0 {
        // If the current bit of exp is 1 (i.e., exp is odd)
        if exp % 2 == 1 {
            result *= base;
        }
        // Square the base for the next iteration.
        base *= base;
        // Right shift the exponent to process the next bit.
        exp /= 2;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fast_power_zero_exponent() {
        assert_eq!(fast_power(5, 0), 1);
    }

    #[test]
    fn test_fast_power_one_exponent() {
        assert_eq!(fast_power(5, 1), 5);
    }

    #[test]
    fn test_fast_power_small_numbers() {
        assert_eq!(fast_power(2, 3), 8);
        assert_eq!(fast_power(3, 4), 81);
    }

    #[test]
    fn test_fast_power_larger_numbers() {
        assert_eq!(fast_power(7, 10), 282475249);
        assert_eq!(fast_power(10, 5), 100000);
    }

    #[test]
    #[should_panic(expected = "Exponent must be non-negative")]
    fn test_fast_power_negative_exponent() {
        fast_power(2, u32::MAX);
    }
}