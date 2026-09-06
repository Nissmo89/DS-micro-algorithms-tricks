#include <stdint.h> // For uint64_t

// Calculates base^exp efficiently using exponentiation by squaring.
// This algorithm reduces the number of multiplications from O(exp) to O(log exp).
// It works by processing the bits of the exponent.
//
// If the current bit of the exponent is 1, multiply the result by the current base.
// Then, square the base for the next bit.
//
// Example: 3^10
// exp = 10 (binary 1010)
//
// Iteration 1 (LSB): exp=1010, base=3, result=1
//   - exp is even (LSB is 0). result remains 1.
//   - base becomes 3*3 = 9.
//   - exp becomes 101 (right shift).
//
// Iteration 2: exp=101, base=9, result=1
//   - exp is odd (LSB is 1). result becomes 1*9 = 9.
//   - base becomes 9*9 = 81.
//   - exp becomes 10 (right shift).
//
// Iteration 3: exp=10, base=81, result=9
//   - exp is even (LSB is 0). result remains 9.
//   - base becomes 81*81 = 6561.
//   - exp becomes 1 (right shift).
//
// Iteration 4: exp=1, base=6561, result=9
//   - exp is odd (LSB is 1). result becomes 9*6561 = 59049.
//   - base becomes 6561*6561 (large).
//   - exp becomes 0 (right shift).
//
// Loop terminates. Result is 59049.
uint64_t fast_power_integer(uint64_t base, uint32_t exp) {
    uint64_t result = 1;

    // Handle base cases
    if (exp == 0) return 1;
    if (base == 0) return 0;

    while (exp > 0) {
        // If exp is odd, multiply result with base
        if (exp % 2 == 1) {
            result *= base;
        }
        // Square the base
        base *= base;
        // Divide exp by 2 (integer division)
        exp /= 2;
    }

    return result;
}
