from collections import Counter


class Solution:
    def predictPartyVictory(self, senate: str) -> str:
        sc = Counter(senate)
        sl = list(senate)
        db = []
        i = 0
        while sc["R"] != 0 and sc["D"] != 0:
            if i == len(sl):
                i = 0
            if not sl[i]:
                i += 1
                continue
            if not db:
                db = [sl[i], 1]
                i += 1
                continue
            if db[0] == sl[i]:
                db[1] += 1
                i += 1
                continue

            db[1] -= 1
            sc[sl[i]] -= 1
            sl[i] = None
            i+=1
            if db[1] == 0:
                db = None

        return "Radiant" if sc["R"] != 0 else "Dire"

