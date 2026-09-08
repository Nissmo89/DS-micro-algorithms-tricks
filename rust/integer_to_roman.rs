/// Converts an integer to its Roman numeral representation.
///
/// This function uses a greedy approach by iterating through a predefined list of Roman numeral symbols
/// and their corresponding integer values, from largest to smallest. For each symbol, it subtracts
/// its value from the input integer as many times as possible, appending the symbol to the result
/// string each time.
///
/// Constraints: The input integer `num` is guaranteed to be within the range [1, 3999].
///
/// # Arguments
///
/// * `num` - The integer to convert (1-3999).
///
/// # Returns
///
/// A `String` containing the Roman numeral representation of the integer.
///
/// # Examples
///
/// ```
/// assert_eq!(integer_to_roman(3), "III");
/// assert_eq!(integer_to_roman(58), "LVIII");
/// assert_eq!(integer_to_roman(1994), "MCMXCIV");
/// ```
pub fn integer_to_roman(mut num: i32) -> String {
    // Define the Roman numeral symbols and their values in descending order.
    // Includes subtractive pairs like CM, CD, XC, XL, IX, IV for efficiency.
    let values = [
        1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1,
    ];
    let symbols = [
        "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I",
    ];

    let mut roman = String::new();

    // Iterate through the values and symbols
    for i in 0..values.len() {
        // While the current value can be subtracted from num
        while num >= values[i] {
            // Append the corresponding symbol to the result
            roman.push_str(symbols[i]);
            // Subtract the value from num
            num -= values[i];
        }
        // Optimization: If num becomes 0, we can break early.
        if num == 0 {
            break;
        }
    }

    roman
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_to_roman_basic() {
        assert_eq!(integer_to_roman(3), "III");
        assert_eq!(integer_to_roman(58), "LVIII");
    }

    #[test]
    fn test_integer_to_roman_complex() {
        assert_eq!(integer_to_roman(1994), "MCMXCIV");
        assert_eq!(integer_to_roman(2023), "MMXXIII");
    }

    #[test]
    fn test_integer_to_roman_edge_cases() {
        assert_eq!(integer_to_roman(1), "I");
        assert_eq!(integer_to_roman(3999), "MMMCMXCIX");
    }

    #[test]
    fn test_integer_to_roman_subtractive() {
        assert_eq!(integer_to_roman(4), "IV");
        assert_eq!(integer_to_roman(9), "IX");
        assert_eq!(integer_to_roman(40), "XL");
        assert_eq!(integer_to_roman(90), "XC");
        assert_eq!(integer_to_roman(400), "CD");
        assert_eq!(integer_to_roman(900), "CM");
    }
}