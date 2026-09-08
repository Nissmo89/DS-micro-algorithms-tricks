package main

// IntegerLog2 calculates the integer logarithm base 2 of a `uint64` integer.
// This function returns the largest integer `k` such that `2^k <= n`.
// It utilizes bit manipulation to efficiently find the position of the most significant bit (MSB).
// The position of the MSB is the integer log base 2.
//
// For `n = 0`, the behavior is undefined or could be considered negative infinity.
// This implementation returns 0 for `n = 0` for simplicity, though a more robust
// version might panic or return an error.
func IntegerLog2(n uint64) uint64 {
	if n == 0 {
		return 0 // Or panic!, depending on desired behavior for log(0)
	}

	var log uint64 = 0
	// Repeatedly divide n by 2 (right shift) until it becomes 1.
	// The number of shifts is the integer log base 2.
	for n > 1 {
		n >>= 1 // Equivalent to n = n / 2
		log++
	}
	return log

	/*
	// A more optimized version using compiler built-ins (highly efficient if available):
	// For GCC/Clang:
	// if n == 0 { return 0 } // Or panic
	// return 63 - bits.LeadingZeros64(n)
	*/
}
