#include <stddef.h> // For size_t

// Finds the missing number in an array containing numbers from 0 to n, where n is the size of the array.
// This method uses the XOR property: a ^ a = 0 and a ^ 0 = a.
// The idea is to XOR all numbers from 0 to n with all numbers present in the array.
// All numbers that are present in both sequences will cancel out (XORed with themselves), leaving only the missing number.
//
// Example:
// Array: [3, 0, 1], n = 3 (size of array)
// Expected numbers: 0, 1, 2, 3
//
// XOR of expected numbers: 0 ^ 1 ^ 2 ^ 3
// XOR of array numbers:    3 ^ 0 ^ 1
//
// Result = (0 ^ 1 ^ 2 ^ 3) ^ (3 ^ 0 ^ 1)
//        = (0^0) ^ (1^1) ^ 2 ^ (3^3)  (rearranging due to associativity/commutativity)
//        = 0 ^ 0 ^ 2 ^ 0
//        = 2
int find_missing_number(int arr[], size_t n) {
    int missing = n; // Initialize with n, as the loop goes up to n-1

    for (size_t i = 0; i < n; ++i) {
        missing ^= i;
        missing ^= arr[i];
    }

    return missing;
}
