package main

import "fmt"

func rabinKarp(text, pattern string) []int {
    n, m := len(text), len(pattern)
    if m == 0 || n < m {
        return nil
    }
    const base = 256
    const mod = 101
    var h int = 1
    for i := 0; i < m-1; i++ {
        h = (h * base) % mod
    }
    var p, t int
    for i := 0; i < m; i++ {
        p = (base*p + int(pattern[i])) % mod
        t = (base*t + int(text[i])) % mod
    }
    var res []int
    for i := 0; i <= n-m; i++ {
        if p == t {
            if text[i:i+m] == pattern {
                res = append(res, i)
            }
        }
        if i < n-m {
            t = (base*(t-int(text[i])*h) + int(text[i+m])) % mod
            if t < 0 {
                t += mod
            }
        }
    }
    return res
}

func main() {
    text := "ABABDABACDABABCABAB"
    pattern := "ABABCABAB"
    fmt.Println(rabinKarp(text, pattern))
}