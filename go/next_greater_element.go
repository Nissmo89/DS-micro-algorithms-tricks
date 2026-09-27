package main

import (
    "fmt"
)

// nextGreaterElement returns the next greater element for each item in arr.
// If none exists, -1 is used.
func nextGreaterElement(arr []int) []int {
    n := len(arr)
    res := make([]int, n)
    stack := make([]int, 0, n)
    for i := n - 1; i >= 0; i-- {
        for len(stack) > 0 && stack[len(stack)-1] <= arr[i] {
            stack = stack[:len(stack)-1]
        }
        if len(stack) == 0 {
            res[i] = -1
        } else {
            res[i] = stack[len(stack)-1]
        }
        stack = append(stack, arr[i])
    }
    return res
}

func main() {
    arr := []int{4, 5, 2, 25}
    fmt.Println("Next greater elements:", nextGreaterElement(arr))
}
