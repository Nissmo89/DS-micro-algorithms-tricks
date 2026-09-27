package main

import (
    "fmt"
)

// countSubarraysWithSumK returns the number of continuous subarrays that sum to k.
func countSubarraysWithSumK(arr []int, k int) int {
    prefix := 0
    count := 0
    freq := map[int]int{0: 1}
    for _, v := range arr {
        prefix += v
        if c, ok := freq[prefix-k]; ok {
            count += c
        }
        freq[prefix]++
    }
    return count
}

func main() {
    arr := []int{1, 2, 3}
    k := 3
    fmt.Println("Number of subarrays with sum", k, ":", countSubarraysWithSumK(arr, k))
}
