public class IntegerToRomanConverterOptimized {

    // Arrays to store Roman numeral symbols and their corresponding integer values.
    // These are ordered greedily from largest to smallest, including subtractive notations.
    private static final int[] VALUES = {1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1};
    private static final String[] SYMBOLS = {"M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"};

    /**
     * Converts an integer to its Roman numeral representation using optimized lookup arrays.
     *
     * This method employs a greedy strategy. It iterates through the predefined values and symbols,
     * starting from the largest. For each value, it checks how many times it can be subtracted from
     * the input number and appends the corresponding symbol that many times to the result.
     *
     * Constraints: The input integer `num` is guaranteed to be within the range [1, 3999].
     *
     * @param num The integer to convert.
     * @return The Roman numeral representation of the integer.
     */
    public static String toRoman(int num) {
        StringBuilder roman = new StringBuilder();

        // Iterate through the values and symbols
        for (int i = 0; i < VALUES.length; i++) {
            // While the current value can be subtracted from num
            while (num >= VALUES[i]) {
                // Append the corresponding symbol to the result
                roman.append(SYMBOLS[i]);
                // Subtract the value from num
                num -= VALUES[i];
            }
            // Optimization: If num becomes 0, we can break early.
            if (num == 0) {
                break;
            }
        }

        return roman.toString();
    }
}
