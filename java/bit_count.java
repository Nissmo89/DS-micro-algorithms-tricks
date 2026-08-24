public class BitCounter {

    /**
     * Counts the number of set bits (1s) in the binary representation of an integer.
     *
     * This method uses Brian Kernighan's algorithm, which is efficient because
     * it iterates only as many times as there are set bits. In each iteration,
     * it unsets the least significant set bit using the operation `n = n & (n - 1)`.
     *
     * Example:
     * n = 13 (binary 1101)
     * Iteration 1: n = 1101 & 1100 = 1100 (12), count = 1
     * Iteration 2: n = 1100 & 1011 = 1000 (8),  count = 2
     * Iteration 3: n = 1000 & 0111 = 0000 (0),  count = 3
     *
     * @param n The integer whose set bits are to be counted.
     * @return The number of set bits in n.
     */
    public static int countSetBits(int n) {
        int count = 0;
        while (n > 0) {
            n &= (n - 1); // Unsets the least significant set bit
            count++;
        }
        return count;
    }
}
