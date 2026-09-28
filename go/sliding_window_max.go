package main

import (
    "fmt"
)

func maxSlidingWindow(nums []int, k int) []int {
    if len(nums) == 0 || k <= 0 {
        return []int{}
    }
    n := len(nums)
    res := make([]int, 0, n-k+1)
    deque := make([]int, 0)
    for i := 0; i < n; i++ {
        if len(deque) > 0 && deque[0] <= i-k {
            deque = deque[1:]
        }
        for len(deque) > 0 && nums[deque[len(deque)-1]] <= nums[i] {
            deque = deque[:len(deque)-1]
        }
        deque = append(deque, i)
        if i >= k-1 {
            res = append(res, nums[deque[0]])
        }
    }
    return res
}

func main() {
    nums := []int{1, 3, -1, -3, 5, 3, 6, 7}
    k := 3
    fmt.Println(maxSlidingWindow(nums, k))
}