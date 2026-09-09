#include <stddef.h> // For NULL

// Definition for singly-linked list.
typedef struct ListNode {
    int val;
    struct ListNode *next;
} ListNode;

// Reverses a singly linked list iteratively.
//
// This function takes the head of a singly linked list and reverses it in-place.
// It uses three pointers: `prev`, `current`, and `next_node`.
// `prev` starts as NULL, `current` starts as the head.
// In each step, `current`'s `next` pointer is redirected to `prev`.
// Then, `prev` is updated to `current`, and `current` is updated to `next_node`.
// The process continues until `current` becomes NULL, at which point `prev` points to the new head.
//
// Args:
//     head: The head node of the singly linked list.
//
// Returns:
//     The head node of the reversed linked list.
struct ListNode* reverseListIterative(struct ListNode* head) {
    struct ListNode* prev = NULL;
    struct ListNode* current = head;
    struct ListNode* next_node = NULL;

    while (current != NULL) {
        next_node = current->next; // Store the next node
        current->next = prev;      // Reverse the current node's pointer
        prev = current;            // Move prev one step forward
        current = next_node;       // Move current one step forward
    }

    return prev; // prev is the new head of the reversed list
}
