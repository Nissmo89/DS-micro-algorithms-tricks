def max_subarray_at_least_k(arr, k):
    n = len(arr)
    prefix = [0] * (n + 1)
    for i in range(n):
        prefix[i+1] = prefix[i] + arr[i]
    min_prefix = 0
    max_sum = -10**18
    for i in range(k, n+1):
        min_prefix = min(min_prefix, prefix[i - k])
        max_sum = max(max_sum, prefix[i] - min_prefix)
    return max_sum
