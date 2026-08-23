#include <vector>
#include <cmath>

// Finds the prime factorization of a given positive integer n.
// It uses trial division, optimized by checking divisibility up to sqrt(n).
// The algorithm first handles the factor 2, then iterates through odd numbers.
std::vector<long long> prime_factorization(long long n) {
    std::vector<long long> factors;

    // Handle factor 2 separately
    while (n % 2 == 0) {
        factors.push_back(2);
        n /= 2;
    }

    // Check for odd factors from 3 up to sqrt(n)
    // We only need to check odd numbers since even factors are already handled.
    for (long long i = 3; i * i <= n; i += 2) {
        while (n % i == 0) {
            factors.push_back(i);
            n /= i;
        }
    }

    // If n is still greater than 1 after the loop, it means n itself is a prime factor
    // (larger than sqrt(original n)).
    if (n > 1) {
        factors.push_back(n);
    }

    return factors;
}
