package main

import "strings"

// IntegerToRoman converts an integer to its Roman numeral representation.
//
// This method uses a greedy approach by iterating through a predefined list of Roman numeral symbols
// and their corresponding integer values, from largest to smallest. For each symbol, it subtracts
// its value from the input integer as many times as possible, appending the symbol to the result
// string each time.
//
// Constraints: The input integer `num` is guaranteed to be within the range [1, 3999].
//
// Example:
// Input: 1994
// Output: "MCMXCIV"
func IntegerToRoman(num int) string {
	// Define the Roman numeral symbols and their values in descending order.
	// Includes subtractive pairs like CM, CD, XC, XL, IX, IV for efficiency.
	values := []int{1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1}
	symbols := []string{"M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"}

	var roman strings.Builder

	// Iterate through the values and symbols
	for i := 0; i < len(values) && num > 0; i++ {
		// While the current value can be subtracted from num
		for num >= values[i] {
			// Append the corresponding symbol to the result
			roman.WriteString(symbols[i])
			// Subtract the value from num
			num -= values[i]
		}
	}

	return roman.String()
}
