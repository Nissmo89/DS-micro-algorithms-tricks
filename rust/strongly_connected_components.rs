use std::vec::Vec;

pub fn tarjan_scc(n: usize, edges: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut index = 0usize;
    let mut indices = vec![usize::MAX; n];
    let mut lowlink = vec![0usize; n];
    let mut on_stack = vec![false; n];
    let mut stack = Vec::new();
    let mut sccs = Vec::new();

    fn strongconnect(v: usize,
                      index: &mut usize,
                      indices: &mut Vec<usize>,
                      lowlink: &mut Vec<usize>,
                      on_stack: &mut Vec<bool>,
                      stack: &mut Vec<usize>,
                      edges: &Vec<Vec<usize>>,
                      sccs: &mut Vec<Vec<usize>>) {
        indices[v] = *index;
        lowlink[v] = *index;
        *index += 1;
        stack.push(v);
        on_stack[v] = true;

        for &w in &edges[v] {
            if indices[w] == usize::MAX {
                strongconnect(w, index, indices, lowlink, on_stack, stack, edges, sccs);
                lowlink[v] = lowlink[v].min(lowlink[w]);
            } else if on_stack[w] {
                lowlink[v] = lowlink[v].min(indices[w]);
            }
        }

        if lowlink[v] == indices[v] {
            let mut component = Vec::new();
            loop {
                let w = stack.pop().unwrap();
                on_stack[w] = false;
                component.push(w);
                if w == v { break; }
            }
            sccs.push(component);
        }
    }

    for v in 0..n {
        if indices[v] == usize::MAX {
            strongconnect(v, &mut index, &mut indices, &mut lowlink, &mut on_stack, &mut stack, edges, &mut sccs);
        }
    }

    sccs
}