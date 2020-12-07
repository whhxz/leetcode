from typing import List


class Solution:
    def matrixScore(self, A: List[List[int]]) -> int:
        for i, nums in enumerate(A):
            if nums[0] == 0:
                for j, num in enumerate(nums):
                    nums[j] = num ^ 1
        for i in range(1, len(A[0])):
            t = sum([A[j][i] for j in range(len(A))])
            if t * 2 < len(A):
                for j in range(len(A)):
                    A[j][i] = A[j][i] ^ 1

        return sum([int("".join(str(n) for n in nums),2) for nums in A])

