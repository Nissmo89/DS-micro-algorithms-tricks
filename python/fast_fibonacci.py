import numpy as np

def multiply_matrices(A, B):
    """Multiplies two 2x2 matrices.
    Assumes A and B are 2x2 numpy arrays.
    """
    C = np.zeros((2, 2), dtype=np.int64)
    for i in range(2):
        for j in range(2):
            for k in range(2):
                C[i, j] = (C[i, j] + A[i, k] * B[k, j])
    return C

def matrix_power(M, n):
    """Calculates M^n for a 2x2 matrix M using exponentiation by squaring.
    Assumes M is a 2x2 numpy array and n is a non-negative integer.
    """
    if n == 0:
        return np.array([[1, 0], [0, 1]], dtype=np.int64) # Identity matrix
    elif n == 1:
        return M
    
    # If n is even, M^n = (M^(n/2))^2
    if n % 2 == 0:
        half_power = matrix_power(M, n // 2)
        return multiply_matrices(half_power, half_power)
    # If n is odd, M^n = M * (M^((n-1)/2))^2
    else:
        half_power = matrix_power(M, (n - 1) // 2)
        return multiply_matrices(M, multiply_matrices(half_power, half_power))

def fibonacci_matrix(n):
    """Calculates the n-th Fibonacci number using matrix exponentiation.

    The Fibonacci sequence can be represented by the matrix equation:
    [[F(n+1)], [F(n)]] = [[1, 1], [1, 0]]^n * [[F(1)], [F(0)]]
    where F(0) = 0 and F(1) = 1.

    This method achieves O(log n) time complexity.

    Args:
        n: The index of the Fibonacci number to calculate (non-negative).

    Returns:
        The n-th Fibonacci number.
    """
    if n < 0:
        raise ValueError("Input must be a non-negative integer.")
    if n == 0:
        return 0
    
    # The transformation matrix for Fibonacci sequence
    T = np.array([[1, 1], [1, 0]], dtype=np.int64)
    
    # Calculate T^(n-1)
    # We use n-1 because T^1 gives F(2) and F(1)
    T_pow_n_minus_1 = matrix_power(T, n - 1)
    
    # The result is T_pow_n_minus_1[0, 0] * F(1) + T_pow_n_minus_1[0, 1] * F(0)
    # Since F(1)=1 and F(0)=0, this simplifies to T_pow_n_minus_1[0, 0]
    return T_pow_n_minus_1[0, 0]

# Example usage:
# print(fibonacci_matrix(0))  # Output: 0
# print(fibonacci_matrix(1))  # Output: 1
# print(fibonacci_matrix(2))  # Output: 1
# print(fibonacci_matrix(10)) # Output: 55
# print(fibonacci_matrix(50)) # Output: 12586269025
