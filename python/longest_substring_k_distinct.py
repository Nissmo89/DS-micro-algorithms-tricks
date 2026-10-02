def longest_substring_k_distinct(s, k):
    if k == 0:
        return 0, ""
    left = 0
    counts = {}
    max_len = 0
    max_sub = ""
    for right, ch in enumerate(s):
        counts[ch] = counts.get(ch, 0) + 1
        while len(counts) > k:
            left_ch = s[left]
            counts[left_ch] -= 1
            if counts[left_ch] == 0:
                del counts[left_ch]
            left += 1
        if right - left + 1 > max_len:
            max_len = right - left + 1
            max_sub = s[left:right+1]
    return max_len, max_sub