package main

// FindDuplicate finds the duplicate number in an array `nums` which contains n+1 integers
// where each integer is in the range [1, n] inclusive. It is guaranteed that
// exactly one integer is repeated.
//
// This algorithm treats the array as a linked list where the value at index `i` points to index `nums[i]`.
// Since there's a duplicate number, this structure will inevitably form a cycle.
// Floyd's Tortoise and Hare algorithm is used to detect and find the start of this cycle,
// which corresponds to the duplicate number.
//
// Steps:
// 1. Phase 1: Find the intersection point of the two pointers (tortoise and hare).
//    tortoise moves one step: tortoise = nums[tortoise]
//    hare moves two steps: hare = nums[nums[hare]]
// 2. Phase 2: Find the entrance to the cycle.
//    Reset tortoise to the start (index 0).
//    Move both tortoise and hare one step at a time until they meet.
//    The meeting point is the duplicate number.
//
// IMPORTANT: This algorithm relies on the fact that values are within the index range.
// For an array of size n+1 with values from 1 to n, the values can be directly used as indices (0 to n).
func FindDuplicate(nums []int) int {
	// Phase 1: Find intersection point
	tortoise := nums[0]
	hare := nums[0]

	do {
		tortoise = nums[tortoise]
		hare = nums[nums[hare]]
	} while tortoise != hare

	// Phase 2: Find the entrance to the cycle
	tortoise = nums[0]
	for tortoise != hare {
		tortoise = nums[tortoise]
		hare = nums[hare]
	}

	return hare // or tortoise, they meet at the duplicate number
}
