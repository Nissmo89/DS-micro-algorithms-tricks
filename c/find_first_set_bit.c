#include <limits.h> // For INT_MAX

// Finds the position (0-indexed from the right) of the first set bit (least significant bit) in an integer.
// If the integer is 0, it returns -1 (or some indicator that no bit is set).
//
// This function uses a clever bit manipulation trick:
// `n & -n` isolates the least significant bit (LSB) that is set.
// For example, if n = 12 (binary 1100):
// -n (two's complement) = ~n + 1 = ~(1100) + 1 = 0011 + 1 = 0100
// n & -n = 1100 & 0100 = 0100 (which is 4)
//
// Once the LSB is isolated, we can find its position by taking the logarithm base 2.
// However, a more direct way for integers is to use a lookup table or a series of checks.
// This implementation uses a simple iterative approach to find the position after isolating the LSB.
int find_first_set_bit(int n) {
    if (n == 0) {
        return -1; // No set bit
    }

    // Isolate the least significant bit that is set
    // For n=12 (1100), lsb = 4 (0100)
    int lsb = n & (-n);

    // Find the position of this isolated bit.
    // We can do this by repeatedly right-shifting until it becomes 1,
    // or by checking against powers of 2.
    int position = 0;
    while ((1 << position) != lsb) {
        position++;
        // Safety break for extremely large numbers or unexpected behavior,
        // though for standard int sizes this loop should terminate quickly.
        if (position >= sizeof(int) * CHAR_BIT) {
             return -1; // Should not happen for non-zero n
        }
    }

    return position;

    /*
    // Alternative using logarithm (requires math library and floating point, less efficient for integers):
    // #include <math.h>
    // if (n == 0) return -1;
    // return (int)log2(n & -n);
    */

    /*
    // Another approach using compiler built-ins (highly efficient if available):
    // For GCC/Clang:
    // if (n == 0) return -1;
    // return __builtin_ctz(n); // Count trailing zeros
    */
}
