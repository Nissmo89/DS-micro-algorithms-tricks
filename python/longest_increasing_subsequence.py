def lis(arr):
    import bisect
    tail = []
    for x in arr:
        i = bisect.bisect_left(tail, x)
        if i == len(tail):
            tail.append(x)
        else:
            tail[i] = x
    return len(tail)

# Example usage
if __name__ == "__main__":
    seq = [10, 9, 2, 5, 3, 7, 101, 18]
    print("Length of LIS:", lis(seq))