# abc168 d の検証器。答えが一意でないため、d.yml の match: Checker から呼ぶ。
# cargo-compete が環境変数 INPUT / ACTUAL_OUTPUT に入力・実際の出力のファイルパスを渡す。
# 終了コード 0 で AC、それ以外で WA (理由は stderr に出す)。
import os
import sys
from collections import deque


def main():
    with open(os.environ["INPUT"]) as f:
        it = iter(f.read().split())
    n, m = int(next(it)), int(next(it))
    graph = [set() for _ in range(n)]
    for _ in range(m):
        a, b = int(next(it)) - 1, int(next(it)) - 1
        graph[a].add(b)
        graph[b].add(a)

    dist = [-1] * n
    dist[0] = 0
    queue = deque([0])
    while queue:
        v = queue.popleft()
        for u in graph[v]:
            if dist[u] < 0:
                dist[u] = dist[v] + 1
                queue.append(u)

    with open(os.environ["ACTUAL_OUTPUT"]) as f:
        out = f.read().split()

    # 連結保証があるので正解は常に Yes
    if not out or out[0] != "Yes":
        sys.exit(f"expected Yes, got {out[:1]}")
    if len(out) != n:
        sys.exit(f"expected {n - 1} signposts, got {len(out) - 1}")
    for v in range(1, n):
        p = int(out[v]) - 1
        if p not in graph[v]:
            sys.exit(f"room {v + 1}: {p + 1} is not adjacent")
        if dist[p] != dist[v] - 1:
            sys.exit(f"room {v + 1}: dist {dist[v]} -> {p + 1} has dist {dist[p]}, not {dist[v] - 1}")


main()
