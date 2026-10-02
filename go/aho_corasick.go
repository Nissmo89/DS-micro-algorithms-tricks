package main

import (
    "fmt"
    "strings"
)

// Node represents a state in the Aho-Corasick automaton.
type Node struct {
    children map[byte]*Node
    fail     *Node
    output   []int // indices of patterns ending at this node
}

// Automaton holds the trie root and the patterns.
type Automaton struct {
    root    *Node
    patterns []string
}

// NewAutomaton builds the automaton from the given patterns.
func NewAutomaton(patterns []string) *Automaton {
    a := &Automaton{root: &Node{children: make(map[byte]*Node)}, patterns: patterns}
    // Build trie
    for i, pat := range patterns {
        node := a.root
        for j := 0; j < len(pat); j++ {
            c := pat[j]
            if node.children[c] == nil {
                node.children[c] = &Node{children: make(map[byte]*Node)}
            }
            node = node.children[c]
        }
        node.output = append(node.output, i)
    }
    // Build failure links
    queue := []*Node{}
    // Set fail link of depth-1 nodes to root
    for _, child := range a.root.children {
        child.fail = a.root
        queue = append(queue, child)
    }
    for len(queue) > 0 {
        current := queue[0]
        queue = queue[1:]
        for c, child := range current.children {
            failNode := current.fail
            for failNode != nil && failNode.children[c] == nil {
                failNode = failNode.fail
            }
            if failNode == nil {
                child.fail = a.root
            } else {
                child.fail = failNode.children[c]
            }
            child.output = append(child.output, child.fail.output...)
            queue = append(queue, child)
        }
    }
    return a
}

// Match returns a slice of tuples (patternIndex, position) indicating pattern matches.
func (a *Automaton) Match(text string) [][2]int {
    var results [][2]int
    node := a.root
    for i := 0; i < len(text); i++ {
        c := text[i]
        for node != a.root && node.children[c] == nil {
            node = node.fail
        }
        if node.children[c] != nil {
            node = node.children[c]
        }
        for _, patIdx := range node.output {
            results = append(results, [2]int{patIdx, i - len(a.patterns[patIdx]) + 1})
        }
    }
    return results
}

func main() {
    patterns := []string{"he", "she", "his", "hers"}
    a := NewAutomaton(patterns)
    text := "ahishers"
    matches := a.Match(text)
    for _, m := range matches {
        fmt.Printf("Pattern %s found at position %d\n", patterns[m[0]], m[1])
    }
    // Expected output:
    // Pattern he found at position 5
    // Pattern hers found at position 4
    // Pattern she found at position 3
    // Pattern his found at position 2
}
