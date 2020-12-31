from typing import List


class Solution:
    def eraseOverlapIntervals(self, intervals: List[List[int]]) -> int:
        if not intervals:
            return 0
        intervals.sort(key=lambda x: (x[1], x[0]))
        before = intervals[0]
        res = 0
        for i in range(1, len(intervals)):
            if intervals[i][0] < before[1]:
                res += 1
            else:
                before = intervals[i]
        return res
