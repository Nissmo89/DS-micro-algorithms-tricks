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
// Edge cases (e.g., partitions at the beginning or end of arrays) are handled using negative and positive infinity.
double findMedianSortedArrays(const std::vector<int>& nums1, const std::vector<int>& nums2) {
    // Ensure nums1 is the shorter array for binary search efficiency
    if (nums1.size() > nums2.size()) {
        return findMedianSortedArrays(nums2, nums1);
    }

    int m = nums1.size();
    int n = nums2.size();
    int total_left_size = (m + n + 1) / 2; // Size of the left partition

    int low = 0;
    int high = m;

    while (low <= high) {
        int partition1 = low + (high - low) / 2; // Partition point for nums1
        int partition2 = total_left_size - partition1; // Corresponding partition point for nums2

        // Determine the four boundary elements for the partitions
        int maxLeft1 = (partition1 == 0) ? std::numeric_limits<int>::min() : nums1[partition1 - 1];
        int minRight1 = (partition1 == m) ? std::numeric_limits<int>::max() : nums1[partition1];

        int maxLeft2 = (partition2 == 0) ? std::numeric_limits<int>::min() : nums2[partition2 - 1];
        int minRight2 = (partition2 == n) ? std::numeric_limits<int>::max() : nums2[partition2];

        // Check if the partitions are correct
        if (maxLeft1 <= minRight2 && maxLeft2 <= minRight1) {
            // Correct partitions found
            if ((m + n) % 2 == 0) { // Even number of elements
                return (std::max(maxLeft1, maxLeft2) + std::min(minRight1, minRight2)) / 2.0;
            } else { // Odd number of elements
                return std::max(maxLeft1, maxLeft2);
            }
        } else if (maxLeft1 > minRight2) {
            // partition1 is too large, need to move left in nums1
            high = partition1 - 1;
        } else { // maxLeft2 > minRight1
            // partition1 is too small, need to move right in nums1
            low = partition1 + 1;
        }
    }

    // Should not reach here if input arrays are sorted.
    return 0.0; // Or throw an exception.
}
