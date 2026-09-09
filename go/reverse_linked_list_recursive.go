package main

// ListNode represents a node in a singly linked list.
type ListNode struct {
	Val  int
	Next *ListNode
}

// ReverseListRecursive reverses a singly linked list recursively.
//
// The base case for the recursion is when the list is empty or has only one node,
// in which case the list is already reversed (or trivially reversed).
// Otherwise, the function recursively reverses the rest of the list (from head.Next onwards).
// Once the rest of the list is reversed, the original head node needs to be appended
// to the end of the reversed sublist.
func ReverseListRecursive(head *ListNode) *ListNode {
	// Base case: If the list is empty or has only one node, return it.
	if head == nil || head.Next == nil {
		return head
	}

	// Recursively reverse the rest of the list.
	// `reversedTail` will be the new head of the reversed sublist.
	reversedTail := ReverseListRecursive(head.Next)

	// Now, `head.Next` still points to the original second node.
	// Let the original second node be `node2`. After recursion, `node2` is the tail
	// of the reversed sublist, and its `Next` pointer is nil.
	// We want to make `node2.Next` point back to `head`.
	// `head.Next` is `node2`.
	head.Next.Next = head

	// Set the original head's `Next` pointer to nil, as it will be the new tail.
	head.Next = nil

	// `reversedTail` is the new head of the fully reversed list.
	return reversedTail
}
