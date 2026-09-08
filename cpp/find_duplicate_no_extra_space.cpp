#include <vector>
#include <stdexcept> // For runtime_error

// Finds the duplicate number in a vector `nums` which contains n+1 integers
// where each integer is in the range [1, n] inclusive. It is guaranteed that
// exactly one integer is repeated.
//
// This algorithm uses the array indices and values to create a linked list structure.
// The value at index `i` (`nums[i]`) represents the next node's index.
// Since there's a duplicate, this structure forms a cycle.
// Floyd's Tortoise and Hare algorithm is used to detect and find the start of this cycle,
// which corresponds to the duplicate number.
//
// IMPORTANT: This implementation modifies the input vector by using values as indices.
// The values in the vector are in the range [1, n], and the vector size is n+1.
// This allows `nums[i]` to be used directly as an index.
//
// Time Complexity: O(n)
// Space Complexity: O(1) (modifies input vector)
int findDuplicate(std::vector<int>& nums) {
    if (nums.empty()) {
        // Handle empty input, though problem constraints usually guarantee non-empty.
        throw std::runtime_error("Input vector cannot be empty.");
    }

    // Phase 1: Find the intersection point of the two pointers.
    // Tortoise moves one step, hare moves two steps.
    int tortoise = nums[0];
    int hare = nums[0];

    do {
        tortoise = nums[tortoise];
        hare = nums[nums[hare]];
    } while (tortoise != hare);

    // Phase 2: Find the entrance to the cycle.
    // Reset tortoise to the start (index 0).
    // Move both tortoise and hare one step at a time until they meet.
    // The meeting point is the duplicate number.
    tortoise = nums[0];
    while (tortoise != hare) {
        tortoise = nums[tortoise];
        hare = nums[hare];
    }

    return hare; // or tortoise, they meet at the duplicate number
}
