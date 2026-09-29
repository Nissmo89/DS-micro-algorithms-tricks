package main

import "fmt"

// HopcroftKarp implements the Hopcroft–Karp algorithm for maximum bipartite matching.
// Nodes on the left side are numbered 1..nLeft, nodes on the right side 1..nRight.
// Edges are added from left to right.
// The algorithm runs in O(E sqrt(V)).

type HopcroftKarp struct {
    nLeft, nRight int
    adj           [][]int
    pairU, pairV  []int
    dist          []int
}

func NewHopcroftKarp(nLeft, nRight int) *HopcroftKarp {
    hk := &HopcroftKarp{
        nLeft:  nLeft,
        nRight: nRight,
        adj:    make([][]int, nLeft+1),
        pairU:  make([]int, nLeft+1),
        pairV:  make([]int, nRight+1),
        dist:   make([]int, nLeft+1),
    }
    return hk
}

func (hk *HopcroftKarp) AddEdge(u, v int) {
    hk.adj[u] = append(hk.adj[u], v)
}

func (hk *HopcroftKarp) bfs() bool {
    queue := make([]int, 0, hk.nLeft)
    for u := 1; u <= hk.nLeft; u++ {
        if hk.pairU[u] == 0 {
            hk.dist[u] = 0
            queue = append(queue, u)
        } else {
            hk.dist[u] = -1
        }
    }
    found := false
    for len(queue) > 0 {
        u := queue[0]
        queue = queue[1:]
        for _, v := range hk.adj[u] {
            if hk.pairV[v] == 0 {
                found = true
            } else if hk.dist[hk.pairV[v]] == -1 {
                hk.dist[hk.pairV[v]] = hk.dist[u] + 1
                queue = append(queue, hk.pairV[v])
            }
        }
    }
    return found
}

func (hk *HopcroftKarp) dfs(u int) bool {
    for _, v := range hk.adj[u] {
        if hk.pairV[v] == 0 || (hk.dist[hk.pairV[v]] == hk.dist[u]+1 && hk.dfs(hk.pairV[v])) {
            hk.pairU[u] = v
            hk.pairV[v] = u
            return true
        }
    }
    hk.dist[u] = -1
    return false
}

func (hk *HopcroftKarp) MaxMatching() int {
    matching := 0
    for hk.bfs() {
        for u := 1; u <= hk.nLeft; u++ {
            if hk.pairU[u] == 0 && hk.dfs(u) {
                matching++
            }
        }
    }
    return matching
}

func main() {
    // Example bipartite graph: 4 left nodes, 4 right nodes
    hk := NewHopcroftKarp(4, 4)
    hk.AddEdge(1, 1)
    hk.AddEdge(1, 4)
    hk.AddEdge(2, 1)
    hk.AddEdge(2, 2)
    hk.AddEdge(3, 3)
    hk.AddEdge(4, 3)
    hk.AddEdge(4, 4)
    fmt.Println("Maximum matching size:", hk.MaxMatching())
}