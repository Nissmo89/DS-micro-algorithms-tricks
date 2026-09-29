def find_median_sorted_arrays(nums1, nums2):
    # Ensure nums1 is the smaller array
    A, B = nums1, nums2
    if len(A) > len(B):
        A, B = B, A
    m, n = len(A), len(B)
    low, high = 0, m
    half_len = (m + n + 1) // 2

    while low <= high:
        i = (low + high) // 2
        j = half_len - i

        if i < m and j > 0 and B[j - 1] > A[i]:
            low = i + 1
        elif i > 0 and j < n and A[i - 1] > B[j]:
            high = i - 1
        else:
            # i is perfect
            if i == 0:
                max_of_left = B[j - 1]
            elif j == 0:
                max_of_left = A[i - 1]
            else:
                max_of_left = max(A[i - 1], B[j - 1])

            if (m + n) % 2 == 1:
                return float(max_of_left)

            if i == m:
                min_of_right = B[j]
            elif j == n:
                min_of_right = A[i]
            else:
                min_of_right = min(A[i], B[j])

            return (max_of_left + min_of_right) / 2.0
