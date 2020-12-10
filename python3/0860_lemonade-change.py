from typing import List


class Solution:
    def lemonadeChange(self, bills: List[int]) -> bool:
        tl = [0, 0]
        for b in bills:
            if b == 5:
                tl[0] += 1
            elif b == 10:
                tl[0] -= 1
                tl[1] += 1
            elif b == 20:
                if tl[1] > 0:
                    tl[1] -= 1
                    tl[0] -= 1
                else:
                    tl[0] -= 3
            if tl[0] < 0:
                return False
        return True
