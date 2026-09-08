public class PeakFinder {

    /**
     * Finds a peak element in an array using binary search.
     *
     * A peak element is an element that is strictly greater than its neighbors.
     * The function assumes that nums[-1] = nums[n] = -infinity. This implies that
     * if the array has only one element, it is a peak.
     * If there are multiple peaks, returning any one of them is acceptable.
     *
     * This implementation uses binary search to achieve O(log n) time complexity.
     *
     * @param nums The input array of integers.
     * @return The value of a peak element.
     */
    public static int findPeakElement(int[] nums) {
        int low = 0;
        int high = nums.length - 1;

        while (low < high) {
            int mid = low + (high - low) / 2;

            // If the middle element is smaller than its right neighbor,
            // a peak must be in the right half (mid+1 to high).
            if (nums[mid] < nums[mid + 1]) {
                low = mid + 1;
            } 
            // If the middle element is greater than or equal to its right neighbor,
            // a peak might be nums[mid] or in the left half (low to mid).
            else {
                high = mid;
            }
        }

        // When low == high, we have found a peak element.
        return nums[low];
    }
}
