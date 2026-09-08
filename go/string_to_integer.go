package main

import (
	"math"
	"strings"
)

// Atoi converts a string to a 32-bit signed integer, similar to C's atoi function.
//
// The function follows these rules:
// 1. Skips leading whitespace.
// 2. Checks for an optional sign ('+' or '-').
// 3. Reads subsequent digits until a non-digit character or the end of the string is reached.
// 4. Converts these digits to an integer.
// 5. Handles integer overflow: if the integer is out of the 32-bit signed integer range [-2^31, 2^31 - 1],
//    it clamps the value to `math.MinInt32` or `math.MaxInt32` respectively.
func Atoi(str string) int {
	str = strings.TrimSpace(str) // 1. Skip leading whitespace

	if len(str) == 0 {
		return 0
	}

	ssign := 1
	index := 0

	// 2. Check for sign
	if str[index] == '+' || str[index] == '- ' {
		if str[index] == '- ' {
			sign = -1
		}
		index++
	}

	var result int64 = 0 // Use int64 to detect overflow before casting to int

	// 3. Read digits and convert, checking for overflow
	for ; index < len(str); index++ {
		char := str[index]
		if char < '0' || char > '9' {
			break // Stop at the first non-digit character
		}

		digit := int64(char - '0')

		// Check for overflow before updating result
		if sign == 1 && (result > math.MaxInt32/10 || (result == math.MaxInt32/10 && digit > math.MaxInt32%10)) {
			return math.MaxInt32
		}
		if sign == -1 && (-result < math.MinInt32/10 || (-result == math.MinInt32/10 && -digit < math.MinInt32%10)) {
			return math.MinInt32
		}

		result = result*10 + digit
	}

	// Apply sign and cast to int (already clamped)
	return int(result * int64(sign))
}
