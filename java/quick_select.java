import java.util.Random;

public class QuickSelect {

    private static final Random random = new Random();

    /**
     * Finds the k-th smallest element in an unsorted array using the QuickSelect algorithm.
     *
     * QuickSelect is a selection algorithm to find the k-th smallest element in an unordered list.
     * It is related to the QuickSort sorting algorithm. Like QuickSort, it partitions an array
     * around a pivot element. However, instead of recursing into both partitions, it only
     * recurses into the partition that contains the k-th smallest element.
     *
     * Average Time Complexity: O(n)
     * Worst-case Time Complexity: O(n^2) (rare with random pivot selection)
     * Space Complexity: O(log n) on average due to recursion stack, O(n) in worst case.
     *
     * @param nums The array of integers.
     * @param k    The rank of the element to find (1-based index, e.g., k=1 for the smallest, k=n for the largest).
     * @return The k-th smallest element.
     */
    public static int findKthSmallest(int[] nums, int k) {
        if (nums == null || nums.length == 0 || k < 1 || k > nums.length) {
            throw new IllegalArgumentException("Invalid input");
        }
        // Convert k to 0-based index for internal use
        return quickSelect(nums, 0, nums.length - 1, k - 1);
    }

    /**
     * Recursive helper function for QuickSelect.
     *
     * @param nums The array.
     * @param left The starting index of the current subarray.
     * @param right The ending index of the current subarray.
     * @param kIndex The 0-based index of the element we are looking for.
     * @return The element at the kIndex position after partitioning.
     */
    private static int quickSelect(int[] nums, int left, int right, int kIndex) {
        if (left == right) {
            return nums[left];
        }

        // Select a random pivot index
        int pivotIndex = left + random.nextInt(right - left + 1);

        // Partition the array around the pivot
        pivotIndex = partition(nums, left, right, pivotIndex);

        // Check if the pivot is the k-th smallest element
        if (kIndex == pivotIndex) {
            return nums[kIndex];
        } else if (kIndex < pivotIndex) {
            // If k is smaller than pivot index, search in the left subarray
            return quickSelect(nums, left, pivotIndex - 1, kIndex);
        } else {
            // If k is larger than pivot index, search in the right subarray
            return quickSelect(nums, pivotIndex + 1, right, kIndex);
        }
    }

    /**
     * Partitions the subarray `nums[left...right]` around the pivot element at `pivotIndex`.
     * Elements smaller than the pivot are moved to its left, and elements larger are moved to its right.
     *
     * @param nums The array.
     * @param left The starting index of the subarray.
     * @param right The ending index of the subarray.
     * @param pivotIndex The index of the pivot element.
     * @return The final index of the pivot element after partitioning.
     */
    private static int partition(int[] nums, int left, int right, int pivotIndex) {
        int pivotValue = nums[pivotIndex];
        // Move pivot to the end
        swap(nums, pivotIndex, right);
        int storeIndex = left;
        
        // Iterate through the subarray (excluding the pivot at the end)
        // Move elements smaller than pivot to the left side
        for (int i = left; i < right; i++) {
            if (nums[i] < pivotValue) {
                swap(nums, storeIndex, i);
                storeIndex++;
            }
        }
        
        // Move pivot to its final sorted place
        swap(nums, storeIndex, right);
        return storeIndex;
    }

    /**
     * Swaps two elements in an array.
     * @param nums The array.
     * @param i The index of the first element.
     * @param j The index of the second element.
     */
    private static void swap(int[] nums, int i, int j) {
        int temp = nums[i];
        nums[i] = nums[j];
        nums[j] = temp;
    }
}
