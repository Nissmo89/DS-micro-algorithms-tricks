class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

def reverse_linked_list_recursive(head):
    """Reverses a singly linked list recursively.

    The base case for the recursion is when the list is empty or has only one node,
    in which case the list is already reversed (or trivially reversed).
    Otherwise, the function recursively reverses the rest of the list (from head.next onwards).
    Once the rest of the list is reversed, the original head node needs to be appended
    to the end of the reversed sublist.

    Args:
        head: The head node of the singly linked list.

    Returns:
        The head node of the reversed linked list.
    """
    # Base case: If the list is empty or has only one node, return it.
    if not head or not head.next:
        return head

    # Recursively reverse the rest of the list.
    # `reversed_tail` will be the new head of the reversed sublist.
    reversed_tail = reverse_linked_list_recursive(head.next)

    # Now, `head.next` still points to the original second node.
    # Let the original second node be `node2`. After recursion, `node2` is the tail
    # of the reversed sublist, and its `next` pointer is None.
    # We want to make `node2.next` point back to `head`.
    # `head.next` is `node2`.
    head.next.next = head

    # Set the original head's `next` pointer to None, as it will be the new tail.
    head.next = None

    # `reversed_tail` is the new head of the fully reversed list.
    return reversed_tail

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

# # Reverse the list recursively
# reversed_head = reverse_linked_list_recursive(head)
# print("Reversed list:")
# print_linked_list(reversed_head) # Output: 5 -> 4 -> 3 -> 2 -> 1
