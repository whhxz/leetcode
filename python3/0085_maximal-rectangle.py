from typing import List


class Solution:
    # 先遍历，求每个数为低所在列的高度，
    # 求每个数宽度，计算面积
    def maximalRectangle(self, matrix: List[List[str]]) -> int:
        if len(matrix) == 0 or len(matrix[0]) == 0:
            return 0
        row = len(matrix)
        col = len(matrix[0])
        for i in range(col):
            matrix[0][i] = int(matrix[0][i])

        def helper(nums: List[int]):
            m = 0
            for i, num in enumerate(nums):
                l = i
                r = i
                while l >= 0 and nums[l] >= num:
                    l -= 1
                while r < len(nums) and nums[r] >= num:
                    r += 1
                m = max(m, ((i - l - 1) + (r - i - 1) + 1) * num)
            return m

        res = helper(matrix[0])
        for i in range(1, row):
            for j in range(col):
                matrix[i][j] = int(matrix[i][j])
                if matrix[i][j] != 0 and matrix[i-1][j] != 0:
                    matrix[i][j] += matrix[i-1][j]
            res = max(res, helper(matrix[i]))
        return res

print(Solution().maximalRectangle([["0","0"]]))
