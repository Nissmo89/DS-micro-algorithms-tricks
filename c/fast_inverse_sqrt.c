// This implementation is a direct translation of the famous fast inverse square root algorithm
// found in Quake III Arena source code. It's an approximation and uses bit manipulation
// to achieve speed, primarily for graphics applications where exact precision isn't critical.
// It operates on single-precision floating-point numbers (float).

#include <stdio.h>
#include <stdint.h> // For uint32_t

// Calculates an approximation of the inverse square root (1/sqrt(x)) of a float.
// Uses the IEEE 754 floating-point representation and a "magic number" for the initial guess.
// A Newton-Raphson iteration can be applied for better precision.
float fast_inverse_sqrt(float number) {
    float x2 = number * 0.5f;
    float approx_inv_sqrt;
    uint32_t i;

    // Convert float to its integer representation (as uint32_t)
    // This reinterprets the bits of the float as an unsigned integer.
    i = *(uint32_t *) &number;

    // The 'magic number' 0x5f3759df is crucial for the approximation.
    // It's derived from the properties of IEEE 754 single-precision floats.
    // The operation `i >> 1` performs a bitwise right shift, effectively halving the exponent.
    // Subtracting from the magic number gives a good initial guess for 1/sqrt(x).
    i = 0x5f3759df - (i >> 1);

    // Convert the integer representation back to a float.
    // This reinterprets the integer bits as a float, giving the initial approximation.
    approx_inv_sqrt = *(float *) &i;

    // Optional: One iteration of Newton's method for improved accuracy
    // approx_inv_sqrt = approx_inv_sqrt * (1.5f - x2 * approx_inv_sqrt * approx_inv_sqrt);

    return approx_inv_sqrt;
}

/*
// Example usage:
int main() {
    float num = 16.0f;
    float invSqrt = fast_inverse_sqrt(num);
    printf("Approximation of 1/sqrt(%f): %f\n", num, invSqrt);
    printf("Actual 1/sqrt(%f): %f\n", num, 1.0f / sqrtf(num));

    num = 2.0f;
    invSqrt = fast_inverse_sqrt(num);
    printf("Approximation of 1/sqrt(%f): %f\n", num, invSqrt);
    printf("Actual 1/sqrt(%f): %f\n", num, 1.0f / sqrtf(num));

    return 0;
}
*/
