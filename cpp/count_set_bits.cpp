#include <iostream>

// Counts the number of set bits (1s) in the binary representation of an integer.
// This method uses Brian Kernighan's algorithm, which is efficient because
// it iterates only as many times as there are set bits. In each iteration,
// it unsets the least significant set bit using the operation `n = n & (n - 1)`.
//
// Example:
// n = 13 (binary 1101)
// Iteration 1: n = 1101 & 1100 = 1100 (12), count = 1
// Iteration 2: n = 1100 & 1011 = 1000 (8),  count = 2
// Iteration 3: n = 1000 & 0111 = 0000 (0),  count = 3
// Loop terminates. Result is 3.
int countSetBits(int n) {
    int count = 0;
    while (n > 0) {
        n &= (n - 1); // Unsets the least significant set bit
        count++;
    }
    return count;
}

/*
// Example usage:
int main() {
    std::cout << "Number of set bits in 13: " << countSetBits(13) << std::endl; // Output: 3
    std::cout << "Number of set bits in 7: " << countSetBits(7) << std::endl;   // Output: 3
    std::cout << "Number of set bits in 0: " << countSetBits(0) << std::endl;   // Output: 0
    std::cout << "Number of set bits in 16: " << countSetBits(16) << std::endl; // Output: 1
    return 0;
}
*/
