#include <iostream>

// Swaps two integer values without using a temporary variable.
// This is achieved using the properties of the bitwise XOR operation:
// 1. x ^ x = 0
// 2. x ^ 0 = x
// 3. XOR is associative and commutative.
//
// Steps:
// 1. a = a ^ b;  (a now holds the combined bits of original a and b)
// 2. b = a ^ b;  (b becomes (a ^ b) ^ b = a ^ (b ^ b) = a ^ 0 = a. So b now holds original a's value)
// 3. a = a ^ b;  (a becomes (a ^ b) ^ a = (a ^ a) ^ b = 0 ^ b = b. So a now holds original b's value)
void swap_xor(int& a, int& b) {
    if (a == b) return; // No need to swap if values are the same
    a = a ^ b;
    b = a ^ b;
    a = a ^ b;
}

/*
// Example usage:
int main() {
    int x = 5; // Binary: 0101
    int y = 10; // Binary: 1010

    std::cout << "Before swap: x = " << x << ", y = " << y << std::endl;
    swap_xor(x, y);
    std::cout << "After swap: x = " << x << ", y = " << y << std::endl;
    // Expected output: After swap: x = 10, y = 5

    int z = 7;
    swap_xor(z, z);
    std::cout << "After self-swap: z = " << z << std::endl;
    // Expected output: After self-swap: z = 7

    return 0;
}
*/
