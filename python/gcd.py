def gcd(a, b):
    """Calculates the greatest common divisor (GCD) of two non-negative integers
    using the Euclidean algorithm.

    The algorithm is based on the principle that the greatest common divisor
    of two numbers does not change if the larger number is replaced by its
    difference with the smaller number. This process is repeated until one
    of the numbers becomes zero, at which point the other number is the GCD.
    A more efficient version uses the modulo operator.

    Args:
        a: The first non-negative integer.
        b: The second non-negative integer.

    Returns:
        The greatest common divisor of a and b.
    """
    while b:
        a, b = b, a % b
    return a

# Example usage:
# print(gcd(48, 18))  # Output: 6
# print(gcd(101, 103)) # Output: 1
# print(gcd(0, 5))    # Output: 5
