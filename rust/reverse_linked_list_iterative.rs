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

/// Reverses a singly linked list iteratively.
///
/// This function takes the head of a singly linked list and reverses it in-place.
/// It uses three pointers: `prev`, `current`, and `next_node`.
/// `prev` starts as None, `current` starts as the head.
/// In each step, `current`'s `next` pointer is redirected to `prev`.
/// Then, `prev` is updated to `current`, and `current` is updated to `next_node`.
/// The process continues until `current` becomes None, at which point `prev` points to the new head.
///
/// # Arguments
///
/// * `head` - An `Option<Box<ListNode>>` representing the head of the singly linked list.
///
/// # Returns
///
/// An `Option<Box<ListNode>>` representing the head of the reversed linked list.
pub fn reverse_linked_list_iterative(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    let mut current = head;

    while let Some(mut current_node) = current {
        let next_node = current_node.next.take(); // Store the next node and detach it
        current_node.next = prev;               // Reverse the current node's pointer
        prev = Some(current_node);              // Move prev one step forward
        current = next_node;                    // Move current one step forward
    }

    prev // prev is the new head of the reversed list
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
        let reversed = reverse_linked_list_iterative(head);
        assert_eq!(list_to_vec(reversed), vec![]);
    }

    #[test]
    fn test_reverse_single_node() {
        let head = create_list(vec![1]);
        let reversed = reverse_linked_list_iterative(head);
        assert_eq!(list_to_vec(reversed), vec![1]);
    }

    #[test]
    fn test_reverse_multiple_nodes() {
        let head = create_list(vec![1, 2, 3, 4, 5]);
        let reversed = reverse_linked_list_iterative(head);
        assert_eq!(list_to_vec(reversed), vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_reverse_two_nodes() {
        let head = create_list(vec![1, 2]);
        let reversed = reverse_linked_list_iterative(head);
        assert_eq!(list_to_vec(reversed), vec![2, 1]);
    }
}