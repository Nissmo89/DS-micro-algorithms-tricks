def fast_exponentiation(base, exp):
    """Calculates base^exp efficiently using binary exponentiation (exponentiation by squaring).

    This algorithm reduces the number of multiplications from O(exp) to O(log exp).
    It works by processing the bits of the exponent.

    Args:
        base: The base number.
        exp: The exponent (must be a non-negative integer).

    Returns:
        The result of base^exp.

    Raises:
        ValueError: If the exponent is negative.
    """
    if exp < 0:
        raise ValueError("Exponent must be non-negative.")
    if exp == 0:
        return 1
    if base == 0:
        return 0

    result = 1
    while exp > 0:
        # If the current bit of exp is 1 (i.e., exp is odd)
        if exp % 2 == 1:
            result *= base
        
        # Square the base for the next bit
        base *= base
        # Right shift the exponent to process the next bit
        exp //= 2

    return result

# Example usage:
# print(fast_exponentiation(2, 10))  # Output: 1024
# print(fast_exponentiation(3, 5))   # Output: 243
# print(fast_exponentiation(5, 0))   # Output: 1
# print(fast_exponentiation(0, 10))  # Output: 0
# try:
#     fast_exponentiation(2, -3)
# except ValueError as e:
#     print(e) # Output: Exponent must be non-negative.
