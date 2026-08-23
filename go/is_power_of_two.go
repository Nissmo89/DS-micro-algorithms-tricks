package main

// IsPowerOfTwo checks if a given unsigned integer is a power of two.
// A number n is a power of two if and only if it's positive and
// has exactly one bit set in its binary representation. This can be
// efficiently checked by the expression `n > 0 && (n & (n - 1)) == 0`.
//
// For example:
// If n = 8 (binary 1000), then n-1 = 7 (binary 0111).
// n & (n-1) = 1000 & 0111 = 0000.
// If n = 6 (binary 0110), then n-1 = 5 (binary 0101).
// n & (n-1) = 0110 & 0101 = 0100 (not zero).
func IsPowerOfTwo(n uint64) bool {
	return n > 0 && (n&(n-1)) == 0
}
