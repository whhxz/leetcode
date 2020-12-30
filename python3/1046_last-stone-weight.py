import heapq
from typing import List


class Solution:
    def lastStoneWeight(self, stones: List[int]) -> int:
        stones = [-n for n in stones]
        heapq.heapify(stones)
        while len(stones) > 1:
            n1 = heapq.heappop(stones)
            n2 = heapq.heappop(stones)
            n = abs(n1 - n2)
            if n != 0:
                heapq.heappush(stones, -n)
        return -stones[0] if len(stones) != 0 else 0

