package bit

// BIT represents a Binary Indexed Tree (Fenwick Tree) for integer values.
// It supports point updates and prefix sum queries in O(log n) time.

type BIT struct {
    n    int
    tree []int
}

// NewBIT creates a new BIT of size n. Indices are 1-based.
func NewBIT(n int) *BIT {
    return &BIT{n: n, tree: make([]int, n+1)}
}

// Update adds delta to the element at position idx (1-based).
func (b *BIT) Update(idx, delta int) {
    for idx <= b.n {
        b.tree[idx] += delta
        idx += idx & -idx
    }
}

// Query returns the prefix sum of elements [1, idx] (1-based).
func (b *BIT) Query(idx int) int {
    res := 0
    for idx > 0 {
        res += b.tree[idx]
        idx -= idx & -idx
    }
    return res
}

// RangeQuery returns the sum of elements in the inclusive range [l, r].
func (b *BIT) RangeQuery(l, r int) int {
    return b.Query(r) - b.Query(l-1)
}
