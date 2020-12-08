from typing import List


class Solution:
    def splitIntoFibonacci(self, S: str) -> List[int]:

        def helper(data: List[int], s):
            if s == len(S):
                return True
            ml = len(str(data[-1] + data[-2]))
            if ml + s > len(S) or data[-1] + data[-2] > 2**31 - 1:
                return False
            if data[-1] + data[-2] == int(S[s:s+ml]):
                data.append(int(S[s:s+ml]))
                return helper(data, s + ml)
            return False

        for i in range(1, len(S)-2):
            for j in range(i+1, len(S)-1):
                stack = [int(S[:i]), int(S[i:j])]
                if helper(stack, j):
                    return stack

        return []

