import sys

# --- Fast Input Reading Template ---
# Use sys.stdin.readline() for faster input, especially in competitive programming.
# It reads a whole line at once and is generally faster than input().

# For reading a single integer:
# n = int(sys.stdin.readline())

# For reading a line of space-separated integers:
# line = sys.stdin.readline().split()
# nums = [int(x) for x in line]

# For reading multiple lines of space-separated integers (e.g., T test cases):
# T = int(sys.stdin.readline())
# for _ in range(T):
#     line = sys.stdin.readline().split()
#     nums = [int(x) for x in line]
#     # Process nums here

# For reading a single line of strings:
# words = sys.stdin.readline().split()

# --- Algorithm Placeholder ---
# Replace this section with your actual DSA or algorithmic logic.

def solve():
    # Example: Reading two integers and printing their sum
    try:
        line = sys.stdin.readline().split()
        if not line: # Handle empty input line if necessary
            return
        a = int(line[0])
        b = int(line[1])
        print(a + b)
    except ValueError:
        # Handle cases where input is not convertible to int
        pass
    except IndexError:
        # Handle cases where input line doesn't have enough elements
        pass

# --- Main Execution Block ---
if __name__ == "__main__":
    # If you have multiple test cases, read T first and loop:
    # T = int(sys.stdin.readline())
    # for _ in range(T):
    #     solve()
    
    # If it's a single test case scenario:
    solve()

# --- How to use this template ---
# 1. Copy this code into your Python file (e.g., `main.py`).
# 2. Uncomment and adapt the input reading lines in the `solve()` function
#    or the main execution block based on the problem's input format.
# 3. Replace the example logic inside `solve()` with your algorithm.
# 4. Run your script and provide input via standard input (e.g., `python main.py < input.txt`).
