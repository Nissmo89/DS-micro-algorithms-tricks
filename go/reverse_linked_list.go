package main

// ListNode represents a node in a singly linked list.
type ListNode struct {
	Val  int
	Next *ListNode
}

// ReverseList reverses a singly linked list iteratively.
//
// This function takes the head of a singly linked list and reverses it in-place.
// It uses three pointers: `prev`, `current`, and `nextNode`.
// `prev` starts as nil, `current` starts as the head.
// In each step, `current`'s `Next` pointer is redirected to `prev`.
// Then, `prev` is updated to `current`, and `current` is updated to `nextNode`.
// The process continues until `current` becomes nil, at which point `prev` points to the new head.
func ReverseList(head *ListNode) *ListNode {
	var prev *ListNode // Initially nil
	current := head

	for current != nil {
		nextNode := current.Next // Store the next node
		current.Next = prev      // Reverse the current node's pointer
		prev = current           // Move prev one step forward
		current = nextNode       // Move current one step forward
	}

	return prev // prev is the new head of the reversed list
}
