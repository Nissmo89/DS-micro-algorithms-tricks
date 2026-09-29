def longest_palindromic_substring(s: str) -> str:
    # Transform the string to avoid even/odd length handling
    t = '#' + '#'.join(s) + '#'
    n = len(t)
    p = [0] * n
    center = 0
    right = 0
    for i in range(n):
        mirror = 2 * center - i
        if i < right:
            p[i] = min(right - i, p[mirror])
        # Attempt to expand palindrome centered at i
        while i + p[i] + 1 < n and i - p[i] - 1 >= 0 and t[i + p[i] + 1] == t[i - p[i] - 1]:
            p[i] += 1
        # Update center and right boundary if expanded palindrome is beyond current right
        if i + p[i] > right:
            center = i
            right = i + p[i]
    # Find the maximum element in p
    max_len = max(p)
    center_index = p.index(max_len)
    # Convert back to original string indices
    start = (center_index - max_len) // 2
    return s[start:start + max_len]

if __name__ == "__main__":
    test_str = "babad"
    print("Longest palindromic substring of '{}' is '{}'".format(test_str, longest_palindromic_substring(test_str)))
