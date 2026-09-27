def longest_substring(s: str) -> int:
    last_seen = {}
    start = 0
    max_len = 0
    for i, ch in enumerate(s):
        if ch in last_seen and last_seen[ch] >= start:
            start = last_seen[ch] + 1
        last_seen[ch] = i
        max_len = max(max_len, i - start + 1)
    return max_len

# Example usage:
if __name__ == "__main__":
    test = "pwwkew"
    print(longest_substring(test))  # Output: 3