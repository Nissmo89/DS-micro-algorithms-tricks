class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

def reverse_linked_list_iterative(head):
    """Reverses a singly linked list iteratively.

    This function takes the head of a singly linked list and reverses it in-place.
    It uses three pointers: `prev`, `current`, and `next_node`.
    `prev` starts as None, `current` starts as the head.
    In each step, `current`'s `next` pointer is redirected to `prev`.
    Then, `prev` is updated to `current`, and `current` is updated to `next_node`.
    The process continues until `current` becomes None, at which point `prev` points to the new head.

    Args:
        head: The head node of the singly linked list.

    Returns:
        The head node of the reversed linked list.
    """
    prev = None
    current = head

    while current:
        next_node = current.next  # Store the next node
        current.next = prev       # Reverse the current node's pointer
        prev = current            # Move prev one step forward
        current = next_node       # Move current one step forward

    return prev # prev is the new head of the reversed list

# Example usage:
# # Helper function to create a linked list from a list
# def create_linked_list(arr):
#     if not arr:
#         return None
#     head = ListNode(arr[0])
#     current = head
#     for val in arr[1:]:
#         current.next = ListNode(val)
#         current = current.next
#     return head

# # Helper function to print a linked list
# def print_linked_list(head):
#     vals = []
#     current = head
#     while current:
#         vals.append(str(current.val))
#         current = current.next
#     print(" -> ".join(vals))

# # Create a list: 1 -> 2 -> 3 -> 4 -> 5
# head = create_linked_list([1, 2, 3, 4, 5])
# print("Original list:")
# print_linked_list(head)

# # Reverse the list
# reversed_head = reverse_linked_list_iterative(head)
# print("Reversed list:")
# print_linked_list(reversed_head) # Output: 5 -> 4 -> 3 -> 2 -> 1
