import heapq

class MedianFinder:
    def __init__(self):
        # max heap for lower half (store negatives)
        self.small = []
        # min heap for upper half
        self.large = []

    def add_num(self, num: int) -> None:
        if not self.small or num <= -self.small[0]:
            heapq.heappush(self.small, -num)
        else:
            heapq.heappush(self.large, num)

        # Balance heaps so that len(self.small) == len(self.large) or len(self.small) == len(self.large) + 1
        if len(self.small) > len(self.large) + 1:
            heapq.heappush(self.large, -heapq.heappop(self.small))
        elif len(self.large) > len(self.small):
            heapq.heappush(self.small, -heapq.heappop(self.large))

    def find_median(self) -> float:
        if len(self.small) == len(self.large):
            return (-self.small[0] + self.large[0]) / 2.0
        return float(-self.small[0])
