#include <vector>
#include <algorithm> // For std::reverse

// Helper function to reverse a sub-range of a vector.
void reverse_range(std::vector<int>& nums, int start, int end) {
    while (start < end) {
        std::swap(nums[start], nums[end]);
        start++;
        end--;
    }
}

// Rotates the elements of the vector `nums` to the right by `k` steps.
// This implementation uses the reversal algorithm, which is efficient (O(n) time, O(1) space).
// The algorithm works in three steps:
// 1. Reverse the entire vector.
// 2. Reverse the first k elements.
// 3. Reverse the remaining n-k elements.
// This optimized version ensures k is handled correctly for negative values and multiples of n.
void rotate_optimized(std::vector<int>& nums, int k) {
    int n = nums.size();
    if (n == 0) {
        return;
    }

    // Normalize k: handle cases where k is larger than n or negative.
    k %= n;
    if (k < 0) {
        k += n; // Ensure k is positive after modulo
    }

    if (k == 0) {
        return; // No rotation needed
    }

    // Step 1: Reverse the entire vector
    reverse_range(nums, 0, n - 1);

    // Step 2: Reverse the first k elements
    reverse_range(nums, 0, k - 1);

    // Step 3: Reverse the remaining n-k elements
    reverse_range(nums, k, n - 1);
}
