import sys

# Use sys.stdin.readline for faster input reading in competitive programming.
# This is generally faster than the built-in input() function, especially for large inputs,
# as it reads a whole line at once and avoids some overhead.

def solve():
    # Read the number of test cases (if applicable)
    # T = int(sys.stdin.readline())
    # for _ in range(T):
    #     pass
    
    # Read a single integer
    # n = int(sys.stdin.readline())
    
    # Read a line of space-separated integers
    # line = sys.stdin.readline().split()
    # nums = [int(x) for x in line]
    
    # Read a line of strings
    # words = sys.stdin.readline().split()

    # Example: Reading two integers and printing their sum
    line = sys.stdin.readline().split()
    a = int(line[0])
    b = int(line[1])
    print(a + b)

# To run this, you would typically call solve()
# solve()

# For demonstration, let's simulate input:
# If you were to run this script and type:
# 10 20
# The output would be:
# 30

# This is a template. Replace the example logic with your actual algorithm.
