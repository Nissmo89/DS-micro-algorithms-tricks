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
        Some(mut node) => {
            if node.next.is_none() {
                Some(node)
            } else {
                // Recursively reverse the rest of the list.
                // `rest_reversed` will be the new head of the reversed sublist.
                let mut rest_reversed = reverse_linked_list_recursive(node.next.take());

                // `node` is the current head, its `next` is now None.
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
                return rest_reversed;
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
    fn list_to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32>> {
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
