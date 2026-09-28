def count_inversions(arr):
    def merge_sort(lst):
        n = len(lst)
        if n <= 1:
            return lst, 0
        mid = n // 2
        left, inv_left = merge_sort(lst[:mid])
        right, inv_right = merge_sort(lst[mid:])
        merged = []
        i = j = 0
        inv = 0
        while i < len(left) and j < len(right):
            if left[i] <= right[j]:
                merged.append(left[i])
                i += 1
            else:
                merged.append(right[j])
                inv += len(left) - i
                j += 1
        merged.extend(left[i:])
        merged.extend(right[j:])
        return merged, inv_left + inv_right + inv

    _, inv_count = merge_sort(arr)
    return inv_count

if __name__ == "__main__":
    arr = [2, 4, 1, 3, 5]
    print(count_inversions(arr))  # Output: 3