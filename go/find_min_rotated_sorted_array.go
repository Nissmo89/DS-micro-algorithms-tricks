package main

// FindMin finds the minimum element in a rotated sorted array.
// The array is sorted in ascending order and then rotated at some pivot point.
// This function uses a modified binary search to find the minimum element efficiently.
//
// Example: [4, 5, 6, 7, 0, 1, 2] -> minimum is 0
// Example: [3, 4, 5, 1, 2] -> minimum is 1
// Example: [1, 2, 3, 4, 5] -> minimum is 1 (no rotation or rotated n times)
//
// Time Complexity: O(log n)
// Space Complexity: O(1)
func FindMin(nums []int) int {
	left, right := 0, len(nums)-1

	// If the array is not rotated or has only one element,
	// the first element is the minimum.
	if nums[left] <= nums[right] {
		return nums[left]
	}

	for left < right {
		mid := left + (right-left)/2

		// If nums[mid] is greater than nums[right],
		// it means the pivot (and thus the minimum) is in the right half.
		if nums[mid] > nums[right] {
			left = mid + 1
		} else {
			// If nums[mid] is less than or equal to nums[right],
			// the minimum could be nums[mid] or in the left half.
			// We move right to mid because mid itself could be the minimum.
			right = mid
		}
	}

	// When the loop terminates, left and right will point to the minimum element.
	return nums[left]
}
