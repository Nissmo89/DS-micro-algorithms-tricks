package main

import "math"

// MaxSubarraySum finds the contiguous subarray within `nums` which has the largest sum.
// It implements Kadane's algorithm, which is an efficient dynamic programming approach.
// The algorithm iterates through the array, keeping track of the maximum sum ending at the current position
// and the overall maximum sum found so far.
//
// Time Complexity: O(n)
// Space Complexity: O(1)
func MaxSubarraySum(nums []int) int {
	if len(nums) == 0 {
		return 0 // Or handle as an error/specific value for empty array
	}

	maxSoFar := math.MinInt32 // Initialize with the smallest possible integer
	maxEndingHere := 0

	for _, num := range nums {
		// Decide whether to extend the current subarray or start a new one
		maxEndingHere = maxEndingHere + num

		// If starting a new subarray from the current element gives a larger sum
		if num > maxEndingHere {
			maxEndingHere = num
		}

		// Update the overall maximum sum found so far
		if maxEndingHere > maxSoFar {
			maxSoFar = maxEndingHere
		}
	}

	// Special case: If all numbers are negative, Kadane's algorithm as implemented
	// might return 0 if we initialize maxSoFar to 0. To correctly handle this,
	// we ensure maxSoFar is updated at least once, or we initialize it to the
	// smallest possible int. The above implementation handles this by initializing
	// maxSoFar to math.MinInt32.

	return maxSoFar
}
