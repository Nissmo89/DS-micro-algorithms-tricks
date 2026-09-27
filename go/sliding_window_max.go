package main

import (
    "container/list"
    "fmt"
)

// slidingWindowMax returns the maximum for each window of size k in arr.
func slidingWindowMax(arr []int, k int) []int {
    if k <= 0 {
        return []int{}
    }
    res := make([]int, 0, len(arr)-k+1)
    dq := list.New() // store indices
    for i, v := range arr {
        // Remove indices out of current window
        for dq.Len() > 0 && dq.Front().Value.(int) <= i-k {
            dq.Remove(dq.Front())
        }
        // Remove smaller values from the back
        for dq.Len() > 0 && arr[dq.Back().Value.(int)] <= v {
            dq.Remove(dq.Back())
        }
        dq.PushBack(i)
        if i >= k-1 {
            res = append(res, arr[dq.Front().Value.(int)])
        }
    }
    return res
}

func main() {
    arr := []int{1, 3, -1, -3, 5, 3, 6, 7}
    k := 3
    fmt.Println("Sliding window maximum:", slidingWindowMax(arr, k))
}
