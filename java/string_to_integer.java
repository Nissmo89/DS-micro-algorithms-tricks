public class StringToIntegerConverter {

    /**
     * Converts a string to a 32-bit signed integer, similar to C's atoi function.
     *
     * The function follows these rules:
     * 1. Skips leading whitespace.
     * 2. Checks for an optional sign ('+' or '-').
     * 3. Reads subsequent digits until a non-digit character or the end of the string is reached.
     * 4. Converts these digits to an integer.
     * 5. Handles integer overflow: if the integer is out of the 32-bit signed integer range [-2^31, 2^31 - 1],
     *    it clamps the value to `Integer.MIN_VALUE` or `Integer.MAX_VALUE` respectively.
     *
     * @param str The input string.
     * @return The converted integer, clamped to the 32-bit signed integer range.
     */
    public static int parseInt(String str) {
        if (str == null) {
            return 0; // Or throw an exception
        }

        int index = 0;
        int len = str.length();
        int sign = 1;
        long result = 0; // Use long to detect overflow

        // 1. Skip leading whitespace
        while (index < len && Character.isWhitespace(str.charAt(index))) {
            index++;
        }

        // 2. Check for sign
        if (index < len && (str.charAt(index) == '+' || str.charAt(index) == '-')) {
            sign = (str.charAt(index) == '-') ? -1 : 1;
            index++;
        }

        // 3. Read digits and convert, checking for overflow
        while (index < len && Character.isDigit(str.charAt(index))) {
            int digit = str.charAt(index) - '0';
            
            // Check for overflow before multiplying and adding
            // If result * 10 + digit * sign > Integer.MAX_VALUE or < Integer.MIN_VALUE
            // We check against Integer.MAX_VALUE / 10 and Integer.MIN_VALUE / 10
            if (result > Integer.MAX_VALUE / 10 || (result == Integer.MAX_VALUE / 10 && digit > Integer.MAX_VALUE % 10)) {
                return (sign == 1) ? Integer.MAX_VALUE : Integer.MIN_VALUE;
            }
            
            result = result * 10 + digit;
            index++;
        }

        // Apply sign and clamp the result
        result *= sign;

        if (result > Integer.MAX_VALUE) {
            return Integer.MAX_VALUE;
        }
        if (result < Integer.MIN_VALUE) {
            return Integer.MIN_VALUE;
        }

        return (int) result;
    }
}
