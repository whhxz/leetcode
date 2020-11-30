import collections


class Solution:
    def reorganizeString(self, S: str) -> str:
        ss = collections.Counter(S)

        def find(e):
            max_v = 0
            max_k = None
            for k, v in ss.items():
                if k != e and v > max_v:
                    max_v = v
                    max_k = k
            if max_v == 1:
                del ss[max_k]
            else:
                ss[max_k] -= 1
            return max_k
        res = [find(None)]
        while ss:
            s = find(res[-1])
            if s:
                res.append(s)
            else:
                return ""
        return "".join(res)
