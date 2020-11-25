class Solution:
    def sortString(self, s: str) -> str:
        tl = [0] * 26
        for c in s:
            tl[ord(c) - 97] += 1
        rl = []
        l = len(s)
        t = 1
        i = -1
        while len(rl) != l:
            if i == 26:
                t = -1
                i = 25
            elif i == -1:
                t = 1
                i = 0
            if tl[i] != 0:
                rl.append(chr(i + 97))
                tl[i] -= 1
            i += t
        return "".join(rl)
