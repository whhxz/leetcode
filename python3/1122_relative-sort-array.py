import collections
from typing import List


class Solution:
    def relativeSortArray(self, arr1: List[int], arr2: List[int]) -> List[int]:
        ms = collections.Counter(arr1)
        res = []
        for num in arr2:
            res += [num] * ms[num]
            del ms[num]
        ts = []
        for k, v in ms.items():
            ts += [k] * v
        ts.sort()
        return res + ts

