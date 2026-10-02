package main

import "fmt"

func longestIncreasingPath(matrix [][]int) int {
    rows, cols := len(matrix), len(matrix[0])
    memo := make([][]int, rows)
    for i := 0; i < rows; i++ {
        memo[i] = make([]int, cols)
    }
    dirs := [][]int{{1,0},{-1,0},{0,1},{0,-1}}
    var dfs func(int,int) int
    dfs = func(r,c int) int {
        if memo[r][c] != 0 {
            return memo[r][c]
        }
        best := 1
        for _, d := range dirs {
            nr, nc := r+d[0], c+d[1]
            if nr>=0 && nr<rows && nc>=0 && nc<cols && matrix[nr][nc] > matrix[r][c] {
                cur := 1 + dfs(nr,nc)
                if cur > best {
                    best = cur
                }
            }
        }
        memo[r][c] = best
        return best
    }
    res := 0
    for i := 0; i < rows; i++ {
        for j := 0; j < cols; j++ {
            if val := dfs(i,j); val > res {
                res = val
            }
        }
    }
    return res
}

func main() {
    matrix := [][]int{{9,9,4},{6,6,8},{2,1,1}}
    fmt.Println(longestIncreasingPath(matrix)) // 4
}
