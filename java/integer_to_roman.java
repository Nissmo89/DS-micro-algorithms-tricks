public class IntegerToRomanConverter {

    /**
     * Converts an integer to its Roman numeral representation.
     *
     * This method uses a greedy approach by iterating through a predefined list of Roman numeral symbols
     * and their corresponding integer values, from largest to smallest. For each symbol, it subtracts
     * its value from the input integer as many times as possible, appending the symbol to the result
     * string each time.
     *
     * Constraints: The input integer `num` is guaranteed to be within the range [1, 3999].
     *
     * Example:
     * Input: 1994
     * Output: "MCMXCIV"
     * Breakdown:
     * M (1000): 1994 - 1000 = 994. Result: "M"
     * CM (900): 994 - 900 = 94. Result: "MCM"
     * XC (90): 94 - 90 = 4. Result: "MCMXC"
     * IV (4): 4 - 4 = 0. Result: "MCMXCIV"
     *
     * @param num The integer to convert.
     * @return The Roman numeral representation of the integer.
     */
    public static String toRoman(int num) {
        // Define the Roman numeral symbols and their values in descending order.
        // Includes subtractive pairs like CM, CD, XC, XL, IX, IV for efficiency.
        int[] values = {1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1};
        String[] symbols = {"M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"};

        StringBuilder roman = new StringBuilder();

        // Iterate through the values and symbols
        for (int i = 0; i < values.length && num > 0; i++) {
            // While the current value can be subtracted from num
            while (num >= values[i]) {
                // Append the corresponding symbol to the result
                roman.append(symbols[i]);
                // Subtract the value from num
                num -= values[i];
            }
        }

        return roman.toString();
    }
}
