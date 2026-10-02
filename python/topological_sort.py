def topological_sort(n, edges):
    """Return a topological ordering of a DAG with n nodes (0..n-1).
    edges is a list of (u, v) tuples representing a directed edge u -> v.
    If the graph has a cycle, return an empty list.
    """
    from collections import deque
    adj = [[] for _ in range(n)]
    indeg = [0] * n
    for u, v in edges:
        adj[u].append(v)
        indeg[v] += 1
    q = deque([i for i, d in enumerate(indeg) if d == 0])
    order = []
    while q:
        u = q.popleft()
        order.append(u)
        for v in adj[u]:
            indeg[v] -= 1
            if indeg[v] == 0:
                q.append(v)
    return order if len(order) == n else []