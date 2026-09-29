use std::collections::VecDeque;

/// Performs 0-1 BFS on a directed graph where each edge has weight 0 or 1.
/// `adj` is an adjacency list: for each node, a vector of (neighbor, weight) tuples.
/// Returns the shortest distance from `start` to `target`, or `i32::MAX` if unreachable.
fn zero_one_bfs(adj: &Vec<Vec<(usize, u8)>>, start: usize, target: usize) -> i32 {
    let n = adj.len();
    let mut dist = vec![i32::MAX; n];
    let mut dq = VecDeque::new();
    dist[start] = 0;
    dq.push_back(start);

    while let Some(u) = dq.pop_front() {
        if u == target { break; }
        for &(v, w) in &adj[u] {
            let nd = dist[u] + w as i32;
            if nd < dist[v] {
                dist[v] = nd;
                if w == 0 {
                    dq.push_front(v);
                } else {
                    dq.push_back(v);
                }
            }
        }
    }
    dist[target]
}

fn main() {
    // Example graph:
    // 0 --0--> 1 --1--> 2
    // 0 --1--> 2
    let adj = vec![
        vec![(1, 0), (2, 1)],
        vec![(2, 1)],
        vec![],
    ];
    let distance = zero_one_bfs(&adj, 0, 2);
    println!("Shortest distance from 0 to 2: {}", distance); // should print 1
}
