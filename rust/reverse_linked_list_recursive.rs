#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

/// Reverses a singly linked list recursively.
///
/// The base case for the recursion is when the list is empty or has only one node,
/// in which case the list is already reversed (or trivially reversed).
/// Otherwise, the function recursively reverses the rest of the list (from head.next onwards).
/// Once the rest of the list is reversed, the original head node needs to be appended
/// to the end of the reversed sublist.
///
/// # Arguments
///
/// * `head` - An `Option<Box<ListNode>>` representing the head of the singly linked list.
///
/// # Returns
///
/// An `Option<Box<ListNode>>` representing the head of the reversed linked list.
pub fn reverse_linked_list_recursive(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    // Base case: If the list is empty or has only one node, return it.
    match head {
        None => None,
        Some(node) => {
            if node.next.is_none() {
                Some(node)
            } else {
                // Recursively reverse the rest of the list.
                // `reversed_tail` will be the new head of the reversed sublist.
                let mut reversed_tail = reverse_linked_list_recursive(node.next.take());

                // Now, `node.next` is None (because we took it).
                // The original `node` is now detached from the rest of the list.
                // The original `node.next` (which was the second node) is now the tail of `reversed_tail`.
                // We need to make the original second node's `next` point back to `node`.
                // This means `reversed_tail.as_mut().unwrap().next` should point to `node`.
                // However, we can access the tail of `reversed_tail` directly via `node.next.next`
                // before we set `node.next` to None.

                // Let's re-think the recursive step more carefully with Rust's ownership.
                // We have `node` (original head) and `node.next` is conceptually the rest.
                // `reverse_linked_list_recursive(node.next.take())` returns the reversed tail.
                // Let `rest_reversed = reverse_linked_list_recursive(node.next.take())`.
                // `rest_reversed` is the new head. Its tail is the original `node.next`.
                // We need `original_node.next.next = original_node`.
                // This is tricky with `Option<Box<ListNode>>`.

                // A cleaner recursive approach in Rust often involves passing mutable references or restructuring.
                // Let's use a common pattern: pass the rest, reverse it, then attach the head.

                // Let's try a slightly different recursive structure that's more idiomatic for Rust's ownership.
                // The classic C/Python recursive approach relies on mutable pointers easily.
                // Rust's approach often involves returning the new head.

                // Revisit the logic: If we have head -> A -> B -> C (None)
                // reverse(A -> B -> C) returns C -> B -> A (None)
                // Now, head->next is A. We want A.next = head. And head.next = None.
                // `reversed_tail` is C.

                // Get mutable access to the tail of the reversed list (which is the original `node.next`)
                if let Some(mut tail_node) = reversed_tail.as_mut() {
                    // The `tail_node` is the head of the reversed rest. Its `next` is currently None.
                    // We need to make `tail_node.next` point to the original `head` node.
                    // But `head` is owned by `node` which we are about to return. This is complex.

                    // Let's use a helper function that returns the new head and modifies pointers.
                    // Or, more simply, let's stick to the iterative approach which is often preferred in Rust for linked lists due to ownership.

                    // For a purely recursive solution that mimics C/Python, it's often done like this:
                    // 1. Base case: empty or single node list.
                    // 2. Recursive step: reverse the tail (head.next).
                    // 3. Let `rest` be the reversed tail. Make `head.next.next = head`.
                    // 4. Make `head.next = None`.
                    // 5. Return `rest`.

                    // Let's implement that logic carefully in Rust:
                    let mut rest_reversed = reverse_linked_list_recursive(node.next.take());
                    // `node` is the current head, `node.next` is now None.
                    // `rest_reversed` is the head of the reversed tail.
                    // We need to find the tail of `rest_reversed` and make its `next` point to `node`.

                    // Find the tail of rest_reversed
                    let mut current_in_reversed = rest_reversed.as_mut();
                    while let Some(curr) = current_in_reversed {
                        if curr.next.is_none() {
                            // Found the tail
                            curr.next = Some(node); // Attach the original head
                            break;
                        }
                        current_in_reversed = curr.next.as_mut();
                    }
                    // If rest_reversed was empty (original list had only head), this logic needs adjustment.
                    // But the base case handles that.
                    return rest_reversed;
                } else {
                    // This case should not happen if node.next was Some initially and the base case works.
                    // If node.next was None, base case handled it.
                    // If node.next was Some, rest_reversed should be Some.
                    Some(node) // Should ideally not reach here, return original node if list had only one element
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to create a linked list from a vector
    fn create_list(vals: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        for &val in vals.iter().rev() {
            let mut new_node = Box::new(ListNode::new(val));
            new_node.next = head;
            head = Some(new_node);
        }
        head
    }

    // Helper to convert a linked list to a vector
    fn list_to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut vec = Vec::new();
        while let Some(node) = head {
            vec.push(node.val);
            head = node.next;
        }
        vec
    }

    #[test]
    fn test_reverse_empty() {
        let head = None;
        let reversed = reverse_linked_list_recursive(head);
        assert_eq!(list_to_vec(reversed), vec![]);
    }

    #[test]
    fn test_reverse_single_node() {
        let head = create_list(vec![1]);
        let reversed = reverse_linked_list_recursive(head);
        assert_eq!(list_to_vec(reversed), vec![1]);
    }

    #[test]
    fn test_reverse_multiple_nodes() {
        let head = create_list(vec![1, 2, 3, 4, 5]);
        let reversed = reverse_linked_list_recursive(head);
        assert_eq!(list_to_vec(reversed), vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_reverse_two_nodes() {
        let head = create_list(vec![1, 2]);
        let reversed = reverse_linked_list_recursive(head);
        assert_eq!(list_to_vec(reversed), vec![2, 1]);
    }
}
