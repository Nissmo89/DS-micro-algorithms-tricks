def binary_search_recursive(arr, low, high, x):
    """Performs a recursive binary search on a sorted array.

    Args:
        arr: The sorted list to search within.
        low: The starting index of the search range.
        high: The ending index of the search range.
        x: The element to search for.

    Returns:
        The index of x in arr if found, otherwise -1.
    """
    if high >= low:
        mid = (high + low) // 2

        # If element is present at the middle itself
        if arr[mid] == x:
            return mid

        # If element is smaller than mid, then it can only
        # be present in left subarray
        elif arr[mid] > x:
            return binary_search_recursive(arr, low, mid - 1, x)

        # Else the element can only be present in right subarray
        else:
            return binary_search_recursive(arr, mid + 1, high, x)

    else:
        # Element is not present in the array
        return -1


# Example usage:
# my_list = [2, 3, 4, 10, 40]
# target = 10
# result = binary_search_recursive(my_list, 0, len(my_list) - 1, target)
# 
# if result != -1:
#     print(f"Element is present at index {result}")
# else:
#     print("Element is not present in array")
