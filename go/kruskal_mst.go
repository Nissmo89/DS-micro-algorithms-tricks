package main

import (
    "sort"
    "fmt"
)

// Edge represents an undirected weighted edge between two vertices.
type Edge struct {
    u, v int
    w    int
}

// DSU (Disjoint Set Union) with path compression and union by rank.
type DSU struct {
    parent []int
    rank   []int
}

func NewDSU(n int) *DSU {
    parent := make([]int, n)
    rank := make([]int, n)
    for i := 0; i < n; i++ {
        parent[i] = i
    }
    return &DSU{parent: parent, rank: rank}
}

func (d *DSU) find(x int) int {
    if d.parent[x] != x {
        d.parent[x] = d.find(d.parent[x])
    }
    return d.parent[x]
}

func (d *DSU) union(x, y int) bool {
    xr, yr := d.find(x), d.find(y)
    if xr == yr {
        return false
    }
    if d.rank[xr] < d.rank[yr] {
        d.parent[xr] = yr
    } else if d.rank[xr] > d.rank[yr] {
        d.parent[yr] = xr
    } else {
        d.parent[yr] = xr
        d.rank[xr]++
    }
    return true
}

// Kruskal returns the total weight of the minimum spanning tree and the list of edges in the MST.
func Kruskal(n int, edges []Edge) (int, []Edge) {
    // Sort edges by weight.
    sort.Slice(edges, func(i, j int) bool { return edges[i].w < edges[j].w })

    dsu := NewDSU(n)
    mstWeight := 0
    mstEdges := []Edge{}

    for _, e := range edges {
        if dsu.union(e.u, e.v) {
            mstWeight += e.w
            mstEdges = append(mstEdges, e)
        }
    }
    return mstWeight, mstEdges
}

// Example usage.
func main() {
    // Graph with 5 vertices (0..4) and weighted edges.
    edges := []Edge{
        {0, 1, 10},
        {0, 2, 6},
        {0, 3, 5},
        {1, 3, 15},
        {2, 3, 4},
    }
    weight, mst := Kruskal(5, edges)
    fmt.Println("Total MST weight:", weight)
    fmt.Println("Edges in MST:")
    for _, e := range mst {
        fmt.Printf("(%d, %d) = %d\n", e.u, e.v, e.w)
    }
}
