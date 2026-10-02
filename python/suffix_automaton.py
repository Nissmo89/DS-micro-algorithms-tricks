class SuffixAutomaton:
    def __init__(self):
        self.next = []
        self.link = []
        self.len = []
        self.occ = []
        self.next.append({})  # state 0
        self.link.append(-1)
        self.len.append(0)
        self.occ.append(0)
        self.last = 0

    def add_char(self, c):
        p = self.last
        cur = len(self.next)
        self.next.append({})
        self.link.append(0)
        self.len.append(self.len[p] + 1)
        self.occ.append(1)
        while p != -1 and c not in self.next[p]:
            self.next[p][c] = cur
            p = self.link[p]
        if p == -1:
            self.link[cur] = 0
        else:
            q = self.next[p][c]
            if self.len[p] + 1 == self.len[q]:
                self.link[cur] = q
            else:
                clone = len(self.next)
                self.next.append(self.next[q].copy())
                self.link.append(self.link[q])
                self.len.append(self.len[p] + 1)
                self.occ.append(0)
                while p != -1 and self.next[p].get(c, -1) == q:
                    self.next[p][c] = clone
                    p = self.link[p]
                self.link[q] = clone
                self.link[cur] = clone
        self.last = cur

    def build(self, s):
        for ch in s:
            self.add_char(ch)

    def longest_repeated_substring(self):
        order = sorted(range(len(self.len)), key=lambda i: self.len[i], reverse=True)
        for v in order:
            if self.link[v] != -1:
                self.occ[self.link[v]] += self.occ[v]
        max_len = 0
        for i in range(1, len(self.len)):
            if self.occ[i] > 1:
                max_len = max(max_len, self.len[i])
        return max_len

# Example usage:
# s = "ababcababc"
# sam = SuffixAutomaton()
# sam.build(s)
# print(sam.longest_repeated_substring())  # Output: 5 ("ababc")