/// Performs an iterative binary search on a sorted slice to find the index of a target element.
///
/// Binary search is an efficient algorithm for finding an item from a sorted list of items.
/// It works by repeatedly dividing in half the portion of the list that could contain the item,
/// until you've narrowed down the possible locations to just one.
///
/// # Arguments
///
/// * `arr` - The sorted slice of `i32` elements to search within.
/// * `target` - The `i32` element to search for.
///
/// # Returns
///
/// An `Option<usize>`: `Some(index)` if the target is found at `index`, or `None` if the target is not present.
pub fn binary_search_iterative(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    // Use arr.len() directly for the upper bound. If arr is empty, len() is 0, and the loop condition `low < high` will be false.
    let mut high = arr.len();

    while low < high {
        // Calculate mid point. Using `low + (high - low) / 2` prevents potential overflow if `low + high` is very large.
        let mid = low + (high - low) / 2;

        // Check if the middle element is the target
        if arr[mid] == target {
            return Some(mid);
        }
        // If target is smaller than mid element, search in the left half
        else if arr[mid] > target {
            high = mid; // `mid` itself is not the target, so we exclude it by setting high to `mid`
        }
        // If target is larger than mid element, search in the right half
        else {
            low = mid + 1; // `mid` is not the target, so we start searching from `mid + 1`
        }
    }

    // If the loop finishes without finding the target, it means the target is not in the slice.
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search_found() {
        let arr = [2, 3, 4, 10, 40];
        assert_eq!(binary_search_iterative(&arr, 10), Some(3));
        assert_eq!(binary_search_iterative(&arr, 2), Some(0));
        assert_eq!(binary_search_iterative(&arr, 40), Some(4));
    }

    #[test]
    fn test_binary_search_not_found() {
        let arr = [2, 3, 4, 10, 40];
        assert_eq!(binary_search_iterative(&arr, 13), None);
        assert_eq!(binary_search_iterative(&arr, 1), None);
        assert_eq!(binary_search_iterative(&arr, 50), None);
    }

    #[test]
    fn test_binary_search_empty() {
        let arr: [i32; 0] = [];
        assert_eq!(binary_search_iterative(&arr, 5), None);
    }

    #[test]
    fn test_binary_search_single_element_found() {
        let arr = [5];
        assert_eq!(binary_search_iterative(&arr, 5), Some(0));
    }

    #[test]
    fn test_binary_search_single_element_not_found() {
        let arr = [5];
        assert_eq!(binary_search_iterative(&arr, 3), None);
    }
}