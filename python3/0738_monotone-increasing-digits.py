class Solution:
    def monotoneIncreasingDigits(self, N: int) -> int:
        if N < 10:
            return N
        i = 1
        sn = [int(c) for c in str(N)]
        stack = [sn[0]]
        while i < len(sn):
            if sn[i] < sn[i - 1]:
                stack[-1] -= 1
                break
            stack.append(sn[i])
            i += 1
        if i == len(sn):
            return N
        i -= 1
        while i >= 1 and stack[i] < stack[i - 1]:
            stack[i-1] -= 1
            i -= 1
        if i == 0 and sn[0] == 1:
            return int("9" * (len(sn) - 1))
        return int("".join([str(n) for n in stack[:i + 1]]) + "9" * (len(sn) - i - 1))

