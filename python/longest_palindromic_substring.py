def longest_palindrome(s: str) -> str:
    if not s:
        return ""
    start, end = 0, 0
    for i in range(len(s)):
        # odd length
        l, r = i, i
        while l >= 0 and r < len(s) and s[l] == s[r]:
            l -= 1
            r += 1
        if r - l - 1 > end - start:
            start, end = l + 1, r - 1
        # even length
        l, r = i, i + 1
        while l >= 0 and r < len(s) and s[l] == s[r]:
            l -= 1
            r += 1
        if r - l - 1 > end - start:
            start, end = l + 1, r - 1
    return s[start:end+1]

if __name__ == "__main__":
    test_str = "babad"
    print(longest_palindrome(test_str))  # Expected: "bab" or "aba"
