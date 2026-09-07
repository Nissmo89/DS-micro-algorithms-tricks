package main

import "math"

// FindMedianSortedArrays finds the median of two sorted slices `nums1` and `nums2`.
// This function implements a binary search approach to find the median in O(log(min(m, n))) time complexity,
// where m and n are the lengths of the slices.
//
// The core idea is to partition the two slices such that:
// 1. The total number of elements in the left partitions equals the total number of elements in the right partitions.
// 2. All elements in the left partitions are less than or equal to all elements in the right partitions.
//
// Let `m` be the length of `nums1` and `n` be the length of `nums2`.
// We perform binary search on the smaller slice (say `nums1`) to find the partition point `partition1`.
// The corresponding partition point for `nums2` is then determined by `partition2 = (m + n + 1) / 2 - partition1`.
//
// We need to satisfy the condition: `maxLeft1 <= minRight2` and `maxLeft2 <= minRight1`.
// If these conditions are met, we've found the correct partitions.
// The median is then calculated based on whether the total number of elements (m + n) is odd or even.
// If odd, the median is `max(maxLeft1, maxLeft2)`.
// If even, the median is `(max(maxLeft1, maxLeft2) + min(minRight1, minRight2)) / 2.0`.
//
// Edge cases (e.g., partitions at the beginning or end of slices) are handled using `math.MinInt64` and `math.MaxInt64`.
func FindMedianSortedArrays(nums1 []int, nums2 []int) float64 {
	// Ensure nums1 is the shorter slice for binary search efficiency
	if len(nums1) > len(nums2) {
		nums1, nums2 = nums2, nums1
	}

	m, n := len(nums1), len(nums2)
	totalLeftSize := (m + n + 1) / 2

	low, high := 0, m

	for low <= high {
		partition1 := low + (high-low)/2
		partition2 := totalLeftSize - partition1

		maxLeft1 := math.MinInt64
		if partition1 != 0 {
			maxLeft1 = float64(nums1[partition1-1])
		}

		minRight1 := math.MaxInt64
		if partition1 != m {
			minRight1 = float64(nums1[partition1])
		}

		maxLeft2 := math.MinInt64
		if partition2 != 0 {
			maxLeft2 = float64(nums2[partition2-1])
		}

		minRight2 := math.MaxInt64
		if partition2 != n {
			minRight2 = float64(nums2[partition2])
		}

		if maxLeft1 <= minRight2 && maxLeft2 <= min_right1 {
			// Correct partitions found
			if (m+n)%2 == 0 { // Even number of elements
				return (max(maxLeft1, maxLeft2) + min(minRight1, minRight2)) / 2.0
			} else { // Odd number of elements
				return max(maxLeft1, maxLeft2)
			}
		} else if maxLeft1 > minRight2 {
			// partition1 is too large, move left in nums1
			high = partition1 - 1
		} else { // maxLeft2 > min_right1
			// partition1 is too small, move right in nums1
			low = partition1 + 1
		}
	}

	// Should not reach here if input slices are sorted.
	return math.NaN()
}

func max(a, b float64) float64 {
	if a > b {
		return a
	}
	return b
}

func min(a, b float64) float64 {
	if a < b {
		return a
	}
	return b
}
