from typing import List


class Solution:
    def allCellsDistOrder(self, R: int, C: int, r0: int, c0: int) -> List[List[int]]:
        nums = [[i,j,abs(i-r0) + abs(j-c0)] for j in range(C) for i in range(R)]
        nums.sort(key=lambda x: x[2])
        return [n[:2] for n in nums]

