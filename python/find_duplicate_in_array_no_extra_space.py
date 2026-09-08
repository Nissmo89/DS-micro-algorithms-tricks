def find_duplicate_in_place(nums):
    """Finds the duplicate number in an array `nums` which contains n+1 integers
    where each integer is in the range [1, n] inclusive. It is guaranteed that
    exactly one integer is repeated.

    This algorithm uses the array indices and values to create a linked list structure.
    The value at index `i` (`nums[i]`) represents the next node's index.
    Since there's a duplicate, this structure forms a cycle.
    Floyd's Tortoise and Hare algorithm is used to detect and find the start of this cycle,
    which corresponds to the duplicate number.

    IMPORTANT: This implementation modifies the input array by using values as indices.
    The values in the array are in the range [1, n], and the array size is n+1.
    This allows `nums[i]` to be used directly as an index.

    Args:
        nums: A list of integers where one number is duplicated and others are unique in range [1, n].

    Returns:
        The duplicate number.
    """
    # Phase 1: Find the intersection point of the two pointers.
    # Tortoise moves one step, hare moves two steps.
    tortoise = nums[0]
    hare = nums[0]

    while True:
        tortoise = nums[tortoise]
        hare = nums[nums[hare]]
        if tortoise == hare:
            break

    # Phase 2: Find the entrance to the cycle.
    # Reset tortoise to the start (index 0).
    # Move both tortoise and hare one step at a time until they meet.
    # The meeting point is the duplicate number.
    tortoise = nums[0]
    while tortoise != hare:
        tortoise = nums[tortoise]
        hare = nums[hare]

    return hare # or tortoise, they meet at the duplicate number

# Example usage:
# print(find_duplicate_in_place([1, 3, 4, 2, 2]))  # Output: 2
# print(find_duplicate_in_place([3, 1, 3, 4, 2]))  # Output: 3
# print(find_duplicate_in_place([1, 1]))         # Output: 1
