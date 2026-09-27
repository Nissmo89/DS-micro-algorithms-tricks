package main

import (
    "container/list"
    "fmt"
)

// LRUCache represents a Least Recently Used cache.
// It holds a fixed capacity and evicts the least
// recently used item when the capacity is exceeded.

type LRUCache struct {
    capacity int
    cache    map[int]*list.Element
    order    *list.List
}

// entry holds the key and value stored in the list.
type entry struct {
    key   int
    value int
}

// NewLRUCache creates an LRUCache with the given capacity.
func NewLRUCache(capacity int) *LRUCache {
    return &LRUCache{
        capacity: capacity,
        cache:    make(map[int]*list.Element),
        order:    list.New(),
    }
}

// Get retrieves the value for the given key and marks the entry as recently used.
// If the key is not present, it returns -1.
func (c *LRUCache) Get(key int) int {
    if elem, ok := c.cache[key]; ok {
        c.order.MoveToFront(elem)
        return elem.Value.(*entry).value
    }
    return -1
}

// Put inserts or updates the value for the given key.
// If the cache exceeds its capacity, it evicts the least
// recently used item.
func (c *LRUCache) Put(key int, value int) {
    if elem, ok := c.cache[key]; ok {
        c.order.MoveToFront(elem)
        elem.Value.(*entry).value = value
        return
    }
    if c.order.Len() == c.capacity {
        back := c.order.Back()
        if back != nil {
            evict := back.Value.(*entry)
            delete(c.cache, evict.key)
            c.order.Remove(back)
        }
    }
    e := &entry{key: key, value: value}
    elem := c.order.PushFront(e)
    c.cache[key] = elem
}

// Simple demo to illustrate usage.
func main() {
    cache := NewLRUCache(2)
    cache.Put(1, 1)
    cache.Put(2, 2)
    fmt.Println(cache.Get(1)) // 1
    cache.Put(3, 3)           // evicts key 2
    fmt.Println(cache.Get(2)) // -1
    cache.Put(4, 4)           // evicts key 1
    fmt.Println(cache.Get(1)) // -1
    fmt.Println(cache.Get(3)) // 3
    fmt.Println(cache.Get(4)) // 4
}
