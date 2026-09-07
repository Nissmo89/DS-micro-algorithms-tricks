import math

def find_median_sorted_arrays(nums1, nums2):
    """Finds the median of two sorted lists `nums1` and `nums2`.

    This function implements a binary search approach to find the median in O(log(min(m, n))) time complexity,
    where m and n are the lengths of the lists.

    The core idea is to partition the two lists such that:
    1. The total number of elements in the left partitions equals the total number of elements in the right partitions.
    2. All elements in the left partitions are less than or equal to all elements in the right partitions.

    Let `m` be the length of `nums1` and `n` be the length of `nums2`.
    We perform binary search on the smaller list (say `nums1`) to find the partition point `partition1`.
    The corresponding partition point for `nums2` is then determined by `partition2 = (m + n + 1) // 2 - partition1`.

    We need to satisfy the condition: `maxLeft1 <= minRight2` and `maxLeft2 <= minRight1`.
    If these conditions are met, we've found the correct partitions.
    The median is then calculated based on whether the total number of elements (m + n) is odd or even.
    If odd, the median is `max(maxLeft1, maxLeft2)`.
    If even, the median is `(max(maxLeft1, maxLeft2) + min(minRight1, minRight2)) / 2.0`.

    Edge cases (e.g., partitions at the beginning or end of lists) are handled using `float('-inf')` and `float('inf')`.

    Args:
        nums1: The first sorted list of integers.
        nums2: The second sorted list of integers.

    Returns:
        The median of the two sorted lists as a float.
    """
    # Ensure nums1 is the shorter list for binary search efficiency
    if len(nums1) > len(nums2):
        nums1, nums2 = nums2, nums1

    m, n = len(nums1), len(nums2)
    total_left_size = (m + n + 1) // 2

    low, high = 0, m

    while low <= high:
        partition1 = low + (high - low) // 2
        partition2 = total_left_size - partition1

        max_left1 = float('-inf') if partition1 == 0 else nums1[partition1 - 1]
        min_right1 = float('inf') if partition1 == m else nums1[partition1]

        max_left2 = float('-inf') if partition2 == 0 else nums2[partition2 - 1]
        min_right2 = float('inf') if partition2 == n else nums2[partition2]

        if max_left1 <= min_right2 and max_left2 <= min_right1:
            # Correct partitions found
            if (m + n) % 2 == 0:  # Even number of elements
                return (max(max_left1, max_left2) + min(min_right1, min_right2)) / 2.0
            else:  # Odd number of elements
                return max(max_left1, max_left2)
        elif max_left1 > min_right2:
            # partition1 is too large, move left in nums1
            high = partition1 - 1
        else:  # max_left2 > min_right1
            # partition1 is too small, move right in nums1
            low = partition1 + 1

    # Should not reach here if input lists are sorted.
    # Returning NaN or raising an error might be more appropriate.
    return math.nan

# Example usage:
# print(find_median_sorted_arrays([1, 3], [2]))  # Output: 2.0
# print(find_median_sorted_arrays([1, 2], [3, 4])) # Output: 2.5
# print(find_median_sorted_arrays([], [1])) # Output: 1.0
# print(find_median_sorted_arrays([2], [])) # Output: 2.0
