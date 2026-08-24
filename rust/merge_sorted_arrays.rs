/// Merges two sorted slices into a new sorted `Vec<i32>`.
///
/// This function takes two slices of `i32` that are already sorted in ascending order
/// and returns a new `Vec<i32>` containing all elements from both slices, also sorted.
/// It uses a two-pointer approach to efficiently merge the elements.
///
/// # Arguments
///
/// * `arr1` - The first sorted slice.
/// * `arr2` - The second sorted slice.
///
/// # Returns
///
/// A new `Vec<i32>` containing the merged and sorted elements.
pub fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let len1 = arr1.len();
    let len2 = arr2.len();
    let mut merged = Vec::with_capacity(len1 + len2);

    let mut i = 0; // Pointer for arr1
    let mut j = 0; // Pointer for arr2

    // Iterate while both arrays have elements to compare
    while i < len1 && j < len2 {
        if arr1[i] <= arr2[j] {
            merged.push(arr1[i]);
            i += 1;
        } else {
            merged.push(arr2[j]);
            j += 1;
        }
    }

    // Append any remaining elements from arr1
    while i < len1 {
        merged.push(arr1[i]);
        i += 1;
    }

    // Append any remaining elements from arr2
    while j < len2 {
        merged.push(arr2[j]);
        j += 1;
    }

    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_basic() {
        let arr1 = [1, 3, 5, 7];
        let arr2 = [2, 4, 6, 8];
        let expected = vec![1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(merge_sorted_arrays(&arr1, &arr2), expected);
    }

    #[test]
    fn test_merge_empty_first() {
        let arr1 = [];
        let arr2 = [2, 4, 6, 8];
        let expected = vec![2, 4, 6, 8];
        assert_eq!(merge_sorted_arrays(&arr1, &arr2), expected);
    }

    #[test]
    fn test_merge_empty_second() {
        let arr1 = [1, 3, 5, 7];
        let arr2 = [];
        let expected = vec![1, 3, 5, 7];
        assert_eq!(merge_sorted_arrays(&arr1, &arr2), expected);
    }

    #[test]
    fn test_merge_both_empty() {
        let arr1 = [];
        let arr2 = [];
        let expected: Vec<i32> = vec![];
        assert_eq!(merge_sorted_arrays(&arr1, &arr2), expected);
    }

    #[test]
    fn test_merge_duplicates() {
        let arr1 = [1, 3, 3, 5];
        let arr2 = [2, 3, 4, 6];
        let expected = vec![1, 2, 3, 3, 3, 4, 5, 6];
        assert_eq!(merge_sorted_arrays(&arr1, &arr2), expected);
    }

    #[test]
    fn test_merge_uneven_lengths() {
        let arr1 = [1, 5, 10];
        let arr2 = [2, 3, 4, 6, 7, 8, 9, 11];
        let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        assert_eq!(merge_sorted_arrays(&arr1, &arr2), expected);
    }
}