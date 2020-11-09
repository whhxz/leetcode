from typing import List


class Solution:
    def kClosest(self, points: List[List[int]], K: int) -> List[List[int]]:
        tl = [(p[0] ** 2 + p[1] ** 2, i) for i, p in enumerate(points)]
        tl.sort()
        return [points[tl[i][1]] for i in range(K)]

