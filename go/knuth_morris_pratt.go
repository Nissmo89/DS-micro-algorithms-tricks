package main

import "fmt"

func buildLPS(pattern string) []int {
    lps := make([]int, len(pattern))
    length := 0
    i := 1
    for i < len(pattern) {
        if pattern[i] == pattern[length] {
            length++
            lps[i] = length
            i++
        } else {
            if length != 0 {
                length = lps[length-1]
            } else {
                lps[i] = 0
                i++
            }
        }
    }
    return lps
}

func kmpSearch(text, pattern string) []int {
    lps := buildLPS(pattern)
    var result []int
    i, j := 0, 0
    for i < len(text) {
        if text[i] == pattern[j] {
            i++
            j++
            if j == len(pattern) {
                result = append(result, i-j)
                j = lps[j-1]
            }
        } else {
            if j != 0 {
                j = lps[j-1]
            } else {
                i++
            }
        }
    }
    return result
}

func main() {
    text := "ababcabcabababd"
    pattern := "ababd"
    matches := kmpSearch(text, pattern)
    fmt.Println("Pattern found at indices:", matches)
}
