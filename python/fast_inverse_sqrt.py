# This implementation is a Python translation of the famous fast inverse square root algorithm
# found in Quake III Arena source code. It's an approximation and uses bit manipulation
# to achieve speed, primarily for graphics applications where exact precision isn't critical.

import math

def fast_inverse_sqrt(number):
    """Approximates the inverse square root (1/sqrt(x)) of a number using bit manipulation.

    This algorithm is known for its speed but provides an approximation, not an exact result.
    It leverages the IEEE 754 floating-point representation and a "magic number" for initial guess.
    A Newton-Raphson iteration can be applied for better precision if needed.

    Args:
        number: A positive floating-point number.

    Returns:
        An approximation of 1/sqrt(number).
    """
    if number <= 0:
        raise ValueError("Input must be a positive number.")

    x2 = number * 0.5
    # The 'magic number' 0x5f3759df is crucial for the approximation.
    # It's derived from the properties of IEEE 754 floating-point numbers.
    # The conversion to int and back to float is a bit manipulation trick.
    # Note: Python's float is typically IEEE 754 double-precision (64-bit), while the original
    # algorithm was for single-precision (32-bit). This adaptation might affect precision.
    # For true single-precision behavior, struct module would be needed.
    
    # Using struct module for single precision float behavior (closer to original algorithm)
    import struct
    
    # Convert float to its integer representation (single precision)
    i = struct.pack('>f', number)
    num_int = struct.unpack('>I', i)[0]
    
    # Apply the magic number operation
    num_int = 0x5f3759df - (num_int >> 1)
    
    # Convert back to float
    approx_inv_sqrt = struct.unpack('>f', struct.pack('>I', num_int))[0]

    # Optional: One iteration of Newton's method for improved accuracy
    # approx_inv_sqrt = approx_inv_sqrt * (1.5 - x2 * approx_inv_sqrt * approx_inv_sqrt)

    return approx_inv_sqrt

# Example usage:
# num = 16.0
# result = fast_inverse_sqrt(num)
# print(f"Approximation of 1/sqrt({num}): {result}")
# print(f"Actual 1/sqrt({num}): {1.0 / math.sqrt(num)}")

# num = 2.0
# result = fast_inverse_sqrt(num)
# print(f"Approximation of 1/sqrt({num}): {result}")
# print(f"Actual 1/sqrt({num}): {1.0 / math.sqrt(num)}")
