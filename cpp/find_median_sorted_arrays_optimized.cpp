#include <vector>
#include <algorithm> // For std::min and std::max
#include <limits>    // For numeric_limits

// Finds the median of two sorted arrays `nums1` and `nums2`.
// This function implements a binary search approach to find the median in O(log(min(m, n))) time complexity,
// where m and n are the lengths of the arrays.
//
// The core idea is to partition the two arrays such that:
// 1. The total number of elements in the left partitions equals the total number of elements in the right partitions.
// 2. All elements in the left partitions are less than or equal to all elements in the right partitions.
//
// Let `m` be the length of `nums1` and `n` be the length of `nums2`.
// We perform binary search on the smaller array (say `nums1`) to find the partition point `partition1`.
// The corresponding partition point for `nums2` is then determined by `partition2 = (m + n + 1) / 2 - partition1`.
//
// We need to satisfy the condition: `maxLeft1 <= minRight2` and `maxLeft2 <= minRight1`.
// If these conditions are met, we've found the correct partitions.
// The median is then calculated based on whether the total number of elements (m + n) is odd or even.
// If odd, the median is `max(maxLeft1, maxLeft2)`.
// If even, the median is `(max(maxLeft1, maxLeft2) + min(minRight1, minRight2)) / 2.0`.
//
// Edge cases (e.g., partitions at the beginning or end of arrays) are handled using `std::numeric_limits<int>::min()` and `std::numeric_limits<int>::max()`.
double findMedianSortedArraysOptimized(const std::vector<int>& nums1, const std::vector<int>& nums2) {
    // Ensure nums1 is the shorter array for binary search efficiency
    const std::vector<int>& arr1 = (nums1.size() <= nums2.size()) ? nums1 : nums2;
    const std::vector<int>& arr2 = (nums1.size() <= nums2.size()) ? nums2 : nums1;

    int m = arr1.size();
    int n = arr2.size();
    int total_left_size = (m + n + 1) / 2; // Size of the left partition

    int low = 0;
    int high = m;

    while (low <= high) {
        int partition1 = low + (high - low) / 2; // Partition point for arr1
        int partition2 = total_left_size - partition1; // Corresponding partition point for arr2

        int max_left1 = (partition1 == 0) ? std::numeric_limits<int>::min() : arr1[partition1 - 1];
        int min_right1 = (partition1 == m) ? std::numeric_limits<int>::max() : arr1[partition1];

        int max_left2 = (partition2 == 0) ? std::numeric_limits<int>::min() : arr2[partition2 - 1];
        int min_right2 = (partition2 == n) ? std::numeric_limits<int>::max() : arr2[partition2];

        // Check if the partitions are correct
        if (max_left1 <= min_right2 && max_left2 <= min_right1) {
            // Correct partitions found
            if ((m + n) % 2 == 0) { // Even number of elements
                return (std::max(max_left1, max_left2) + std::min(min_right1, min_right2)) / 2.0;
            } else { // Odd number of elements
                return std::max(max_left1, max_left2);
            }
        } else if (max_left1 > min_right2) {
            // partition1 is too large, need to move left in arr1
            high = partition1 - 1;
        } else { // max_left2 > min_right1
            // partition1 is too small, need to move right in arr1
            low = partition1 + 1;
        }
    }

    // Should not reach here if input arrays are sorted.
    // Returning 0.0 or throwing an exception might be more appropriate.
    return 0.0; 
}
