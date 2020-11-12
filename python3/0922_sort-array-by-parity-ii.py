from typing import List


class Solution:
    def sortArrayByParityII(self, A: List[int]) -> List[int]:
        res = [0] * len(A)
        i,j = 0,1
        for n in A:
            if n % 2==0:
                res[i] = n
                i+=2
            else:
                res[j] = n
                j+=2
        return res
