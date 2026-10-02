package main

import "fmt"

// SegTree implements a segment tree for range minimum queries with lazy propagation for range additions.
// It supports O(log n) updates and queries.

type SegTree struct {
    n    int
    tree []int
    lazy []int
}

func NewSegTree(arr []int) *SegTree {
    n := len(arr)
    size := 4 * n
    st := &SegTree{n: n, tree: make([]int, size), lazy: make([]int, size)}
    st.build(1, 0, n-1, arr)
    return st
}

func (st *SegTree) build(idx, l, r int, arr []int) {
    if l == r {
        st.tree[idx] = arr[l]
        return
    }
    mid := (l + r) / 2
    st.build(idx*2, l, mid, arr)
    st.build(idx*2+1, mid+1, r, arr)
    st.tree[idx] = min(st.tree[idx*2], st.tree[idx*2+1])
}

func (st *SegTree) push(idx int) {
    if st.lazy[idx] != 0 {
        val := st.lazy[idx]
        st.tree[idx*2] += val
        st.lazy[idx*2] += val
        st.tree[idx*2+1] += val
        st.lazy[idx*2+1] += val
        st.lazy[idx] = 0
    }
}

func (st *SegTree) Update(l, r, val int) {
    st.update(1, 0, st.n-1, l, r, val)
}

func (st *SegTree) update(idx, tl, tr, l, r, val int) {
    if l > tr || r < tl {
        return
    }
    if l <= tl && tr <= r {
        st.tree[idx] += val
        st.lazy[idx] += val
        return
    }
    st.push(idx)
    mid := (tl + tr) / 2
    st.update(idx*2, tl, mid, l, r, val)
    st.update(idx*2+1, mid+1, tr, l, r, val)
    st.tree[idx] = min(st.tree[idx*2], st.tree[idx*2+1])
}

func (st *SegTree) Query(l, r int) int {
    return st.query(1, 0, st.n-1, l, r)
}

func (st *SegTree) query(idx, tl, tr, l, r int) int {
    if l > tr || r < tl {
        return int(^uint(0) >> 1) // Max int
    }
    if l <= tl && tr <= r {
        return st.tree[idx]
    }
    st.push(idx)
    mid := (tl + tr) / 2
    left := st.query(idx*2, tl, mid, l, r)
    right := st.query(idx*2+1, mid+1, tr, l, r)
    if left < right {
        return left
    }
    return right
}

func min(a, b int) int {
    if a < b {
        return a
    }
    return b
}

// Demo usage
func main() {
    arr := []int{5, 2, 6, 3, 1, 7, 4}
    st := NewSegTree(arr)
    fmt.Println("Initial min [1,5]:", st.Query(1, 5)) // 1
    st.Update(2, 4, -3) // add -3 to indices 2..4
    fmt.Println("After update min [1,5]:", st.Query(1, 5))
}
