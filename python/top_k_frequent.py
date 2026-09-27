from collections import Counter
import heapq

def top_k_frequent(nums, k):
    freq = Counter(nums)
    return heapq.nlargest(k, freq.keys(), key=freq.get)

if __name__ == "__main__":
    arr = [1,1,1,2,2,3]
    k = 2
    print("Top", k, "frequent elements:", top_k_frequent(arr, k))