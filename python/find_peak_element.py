def find_peak_element(nums):
    """Finds a peak element in an array using binary search.

    A peak element is an element that is strictly greater than its neighbors.
    The function assumes that nums[-1] = nums[n] = -infinity. This implies that
    if the array has only one element, it is a peak.
    If there are multiple peaks, returning any one of them is acceptable.

    This implementation uses binary search to achieve O(log n) time complexity.
    The logic relies on the fact that if we are at an element `mid`:
    1. If `nums[mid]` is greater than its right neighbor (`nums[mid+1]`), then a peak must exist
       in the left half (including `mid`), because either `nums[mid]` is the peak, or the elements
       to its left are greater, leading to a peak further left.
    2. If `nums[mid]` is less than its right neighbor (`nums[mid+1]`), then a peak must exist
       in the right half (excluding `mid`), because `nums[mid+1]` is greater than `nums[mid]`,
       and the trend will eventually lead to a peak.

    Edge cases like the boundaries of the array are handled by the binary search range.

    Args:
        nums: A list of integers.

    Returns:
        The value of a peak element.
    """
    low, high = 0, len(nums) - 1

    while low < high:
        mid = low + (high - low) // 2

        # If the middle element is smaller than its right neighbor,
        # a peak must be in the right half (mid+1 to high).
        if nums[mid] < nums[mid + 1]:
            low = mid + 1
        # If the middle element is greater than or equal to its right neighbor,
        # a peak might be nums[mid] or in the left half (low to mid).
        else:
            high = mid

    # When low == high, we have found a peak element.
    return nums[low]

# Example usage:
# print(find_peak_element([1, 2, 3, 1]))  # Output: 3
# print(find_peak_element([1, 2, 1, 3, 5, 6, 4])) # Output: 2 or 6
# print(find_peak_element([1])) # Output: 1
