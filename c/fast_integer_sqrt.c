#include <stdint.h>

// Calculates the integer square root of a non-negative integer.
// This method uses a bitwise approach, similar to Newton's method but optimized
// for integer arithmetic and bitwise operations for speed.
// It works by finding the highest set bit and then iteratively refining the guess.
int integer_sqrt(uint32_t n) {
    if (n == 0) return 0;

    // Find the position of the most significant bit (MSB)
    // This gives an initial upper bound for the square root.
    int msb_pos = 0;
    uint32_t temp = n;
    while (temp >>= 1) {
        msb_pos++;
    }

    // Initial guess: MSB of sqrt is at half the MSB position of n.
    // For example, if n is 100 (binary 1100100), MSB is at pos 6.
    // sqrt(100) = 10 (binary 1010), MSB is at pos 3.
    int root = 1 << (msb_pos / 2);

    // Iteratively refine the guess using bitwise operations.
    // The idea is to try setting bits from most significant to least significant,
    // and keep the bit if the squared value does not exceed n.
    // This is analogous to how one might perform long division for square roots.
    int bit = 1 << ((msb_pos % 2 == 0) ? (msb_pos / 2 - 1) : (msb_pos / 2));

    while (bit > 0) {
        int next_root = root + bit;
        if (n >= (uint32_t)next_root * next_root) {
            root = next_root;
        }
        bit >>= 1;
    }

    return root;
}
