/// Converts a string slice to a 32-bit signed integer, similar to C's atoi function.
///
/// This function follows these rules:
/// 1. Skips leading whitespace.
/// 2. Checks for an optional sign ('+' or '-').
/// 3. Reads subsequent digits until a non-digit character or the end of the string is reached.
/// 4. Converts these digits to an integer.
/// 5. Handles integer overflow: if the integer is out of the 32-bit signed integer range [-2^31, 2^31 - 1],
///    it clamps the value to `i32::MIN` or `i32::MAX` respectively.
///
/// # Arguments
///
/// * `s` - The input string slice (`&str`).
///
/// # Returns
///
/// The converted integer, clamped to the 32-bit signed integer range.
pub fn atoi(s: &str) -> i32 {
    let mut chars = s.chars().peekable();
    let mut sign = 1i32;
    let mut result = 0i64; // Use i64 to detect overflow before casting to i32

    // 1. Skip leading whitespace
    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
        } else {
            break;
        }
    }

    // 2. Check for sign
    if let Some(&c) = chars.peek() {
        if c == '+' {
            chars.next();
        } else if c == '-' {
            sign = -1;
            chars.next();
        }
    }

    // 3. Read digits and convert, checking for overflow
    while let Some(&c) = chars.peek() {
        if c.is_digit(10) {
            let digit = c.to_digit(10).unwrap() as i64;
            chars.next(); // Consume the digit

            // Check for overflow before updating result
            // Compare against Integer.MAX_VALUE / 10 and Integer.MIN_VALUE / 10
            if sign == 1 && (result > i32::MAX as i64 / 10 || (result == i32::MAX as i64 / 10 && digit > (i32::MAX % 10) as i64)) {
                return i32::MAX;
            }
            if sign == -1 && (-result < i32::MIN as i64 / 10 || (-result == i32::MIN as i64 / 10 && -digit < (i32::MIN % 10) as i64)) {
                return i32::MIN;
            }

            result = result * 10 + digit;
        } else {
            break; // Stop at the first non-digit character
        }
    }

    // Apply sign and cast to i32 (already clamped)
    (result * sign as i64) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atoi_basic() {
        assert_eq!(atoi("42"), 42);
        assert_eq!(atoi("   -42"), -42);
    }

    #[test]
    fn test_atoi_overflow() {
        assert_eq!(atoi("4193 with words"), 4193);
        assert_eq!(atoi("words and 987"), 0);
        assert_eq!(atoi("-91283472332"), i32::MIN);
        assert_eq!(atoi("91283472332"), i32::MAX);
        assert_eq!(atoi("2147483647"), i32::MAX);
        assert_eq!(atoi("-2147483648"), i32::MIN);
        assert_eq!(atoi("2147483648"), i32::MAX); // Overflow positive
        assert_eq!(atoi("-2147483649"), i32::MIN); // Overflow negative
    }

    #[test]
    fn test_atoi_sign_only() {
        assert_eq!(atoi("+"), 0);
        assert_eq!(atoi("-"), 0);
    }

    #[test]
    fn test_atoi_leading_zeros() {
        assert_eq!(atoi("00000-42a1234"), 0);
        assert_eq!(atoi("0032"), 32);
    }

    #[test]
    fn test_atoi_empty_and_whitespace() {
        assert_eq!(atoi(""), 0);
        assert_eq!(atoi("   "), 0);
    }
}