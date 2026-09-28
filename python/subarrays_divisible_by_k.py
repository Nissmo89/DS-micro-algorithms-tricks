def count_subarrays_divisible_by_k(arr, k):
    prefix_mod = 0
    count = 0
    freq = {0: 1}
    for num in arr:
        prefix_mod = (prefix_mod + num) % k
        count += freq.get(prefix_mod, 0)
        freq[prefix_mod] = freq.get(prefix_mod, 0) + 1
    return count