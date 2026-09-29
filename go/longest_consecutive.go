package main

import (
    "fmt"
)

func longestConsecutive(nums []int) int {
    if len(nums) == 0 {
        return 0
    }
    set := make(map[int]struct{}, len(nums))
    for _, n := range nums {
        set[n] = struct{}{}
    }
    maxLen := 0
    for n := range set {
        if _, ok := set[n-1]; !ok {
            cur, val := n, n
            for {
                if _, ok := set[val+1]; ok {
                    val++
                } else {
                    break
                }
            }
            if curVal := val - cur + 1; curVal > maxLen {
                maxLen = curVal
            }
        }
    }
    return maxLen
}

func main() {
    nums := []int{100, 4, 200, 1, 3, 2}
    fmt.Println(longestConsecutive(nums)) // 4
}
