package main

import "fmt"

func longestPalindromicSubstring(s string) string {
    if len(s) == 0 {
        return ""
    }
    t := make([]rune, 2*len(s)+3)
    t[0] = '^'
    idx := 1
    for _, c := range s {
        t[idx] = '#'
        t[idx+1] = c
        idx += 2
    }
    t[idx] = '#'
    t[idx+1] = '$'
    n := len(t)
    p := make([]int, n)
    center, right := 0, 0
    for i := 1; i < n-1; i++ {
        mirror := 2*center - i
        if i < right {
            if p[mirror] < right-i {
                p[i] = p[mirror]
            } else {
                p[i] = right - i
            }
        }
        for t[i+1+p[i]] == t[i-1-p[i]] {
            p[i]++
        }
        if i+p[i] > right {
            center = i
            right = i + p[i]
        }
    }
    maxLen, centerIndex := 0, 0
    for i := 1; i < n-1; i++ {
        if p[i] > maxLen {
            maxLen = p[i]
            centerIndex = i
        }
    }
    start := (centerIndex - maxLen) / 2
    return s[start : start+maxLen]
}

func main() {
    fmt.Println(longestPalindromicSubstring("babad"))
}
