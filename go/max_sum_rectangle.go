package main

import "fmt"

// maxSumRectangle returns the maximum sum of any sub‑matrix within the given 2D slice.
func maxSumRectangle(matrix [][]int) int {
    if len(matrix) == 0 || len(matrix[0]) == 0 {
        return 0
    }
    rows, cols := len(matrix), len(matrix[0])
    maxSum := matrix[0][0]

    // Iterate over left column boundary.
    for left := 0; left < cols; left++ {
        temp := make([]int, rows)
        // Expand right boundary.
        for right := left; right < cols; right++ {
            // Accumulate column sums into temp.
            for r := 0; r < rows; r++ {
                temp[r] += matrix[r][right]
            }
            // Apply 1D Kadane on temp to find max subarray sum for current column span.
            currentMax := kadane(temp)
            if currentMax > maxSum {
                maxSum = currentMax
            }
        }
    }
    return maxSum
}

// kadane finds the maximum subarray sum in a 1D slice.
func kadane(arr []int) int {
    maxEnding := arr[0]
    maxSoFar := arr[0]
    for i := 1; i < len(arr); i++ {
        if maxEnding < 0 {
            maxEnding = arr[i]
        } else {
            maxEnding += arr[i]
        }
        if maxEnding > maxSoFar {
            maxSoFar = maxEnding
        }
    }
    return maxSoFar
}

func main() {
    matrix := [][]int{{1, 2, -1, -4, -20}, {-8, -3, 4, 2, 1}, {3, 8, 10, 1, 3}, {-4, -1, 1, 7, -6}}
    fmt.Println("Maximum sum rectangle:", maxSumRectangle(matrix))
}
