#include <stddef.h> // For NULL

// Definition for singly-linked list.
typedef struct ListNode {
    int val;
    struct ListNode *next;
} ListNode;

// Reverses a singly linked list recursively.
//
// The base case for the recursion is when the list is empty or has only one node,
// in which case the list is already reversed (or trivially reversed).
// Otherwise, the function recursively reverses the rest of the list (from head->next onwards).
// Once the rest of the list is reversed, the original head node needs to be appended
// to the end of the reversed sublist.
//
// Args:
//     head: The head node of the singly linked list.
//
// Returns:
//     The head node of the reversed linked list.
struct ListNode* reverseListRecursive(struct ListNode* head) {
    // Base case: If the list is empty or has only one node, return it.
    if (head == NULL || head->next == NULL) {
        return head;
    }

    // Recursively reverse the rest of the list.
    // `reversed_tail` will be the new head of the reversed sublist.
    struct ListNode* reversed_tail = reverseListRecursive(head->next);

    // Now, `head->next` still points to the original second node.
    // Let the original second node be `node2`. After recursion, `node2` is the tail
    // of the reversed sublist, and its `next` pointer is NULL.
    // We want to make `node2->next` point back to `head`.
    // `head->next` is `node2`.
    head->next->next = head;

    // Set the original head's `next` pointer to NULL, as it will be the new tail.
    head->next = NULL;

    // `reversed_tail` is the new head of the fully reversed list.
    return reversed_tail;
}
