package main

import (
    "fmt"
    "sort"
)

// BuildSuffixArray constructs the suffix array of s using the doubling algorithm.
func BuildSuffixArray(s string) []int {
    n := len(s)
    sa := make([]int, n)
    rank := make([]int, n)
    tmp := make([]int, n)

    for i := 0; i < n; i++ {
        sa[i] = i
        rank[i] = int(s[i])
    }

    k := 1
    for k < n {
        sort.Slice(sa, func(i, j int) bool {
            if rank[sa[i]] != rank[sa[j]] {
                return rank[sa[i]] < rank[sa[j]]
            }
            ri := -1
            rj := -1
            if sa[i]+k < n {
                ri = rank[sa[i]+k]
            }
            if sa[j]+k < n {
                rj = rank[sa[j]+k]
            }
            return ri < rj
        })

        tmp[sa[0]] = 0
        for i := 1; i < n; i++ {
            prev := sa[i-1]
            curr := sa[i]
            if rank[prev] != rank[curr] ||
                ((prev+k < n && curr+k < n && rank[prev+k] != rank[curr+k]) ||
                    (prev+k >= n && curr+k < n) ||
                    (prev+k < n && curr+k >= n)) {
                tmp[curr] = tmp[prev] + 1
            } else {
                tmp[curr] = tmp[prev]
            }
        }
        copy(rank, tmp)
        k <<= 1
    }
    return sa
}

// BuildLCP constructs the LCP array using Kasai's algorithm.
func BuildLCP(s string, sa []int) []int {
    n := len(s)
    rank := make([]int, n)
    for i, v := range sa {
        rank[v] = i
    }
    lcp := make([]int, n-1)
    h := 0
    for i := 0; i < n; i++ {
        if rank[i] > 0 {
            j := sa[rank[i]-1]
            for i+h < n && j+h < n && s[i+h] == s[j+h] {
                h++
            }
            lcp[rank[i]-1] = h
            if h > 0 {
                h--
            }
        }
    }
    return lcp
}

// CountDistinctSubstrings returns the number of distinct substrings of s.
func CountDistinctSubstrings(s string) int {
    n := len(s)
    sa := BuildSuffixArray(s)
    lcp := BuildLCP(s, sa)
    total := n * (n + 1) / 2
    sumLCP := 0
    for _, v := range lcp {
        sumLCP += v
    }
    return total - sumLCP
}

func main() {
    s := "ababa"
    fmt.Println("String:", s)
    fmt.Println("Suffix Array:", BuildSuffixArray(s))
    fmt.Println("LCP Array:", BuildLCP(s, BuildSuffixArray(s)))
    fmt.Println("Distinct substrings:", CountDistinctSubstrings(s))
}
