# std::collections チートシート

`BTreeMap` / `BTreeSet` / `BinaryHeap` / `HashMap` / `HashSet` の使い分けと定石。
本文のコードは rustc 1.89.0（ジャッジと同一）でコンパイル・実行を確認済み。

テンプレートの先頭で以下が `use` 済みなので、そのまま書き始められる。

```rust
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
```

## 選び方

| 欲しいもの | 使うもの | 計算量 |
| --- | --- | --- |
| キー→値、順序いらない | `HashMap` | 平均 O(1) |
| 存在判定だけ、順序いらない | `HashSet` | 平均 O(1) |
| ソート順・範囲検索・前後の要素 | `BTreeMap` / `BTreeSet` | O(log n) |
| 最大 or 最小を取り出すだけ | `BinaryHeap` | push/pop O(log n), peek O(1) |

判断軸は 2 つだけ。

- **順序が要るか** — 要るなら BTree 系、要らないなら Hash 系（定数倍が速い）
- **全順序が要るか極値だけか** — 極値だけなら `BinaryHeap` が最速

さらに細かい切り分け:

- **取り出した結果をまた入れる**（貪欲マージ、ダイクストラ）→ `BinaryHeap`
- **任意の値を消す**（多重集合からの削除）→ `BTreeMap<T, usize>` か `BTreeSet<(T, idx)>`
- **キーが `0..N` の整数** → コレクションより `Vec` の方が桁違いに速い。Hash 系は「キーが疎・非整数」のときに使う

`BTreeSet<T>` は実装上 `BTreeMap<T, ()>` の薄いラッパで、順序の扱いは完全に同一。
どちらも「挿入時に並べ替える」のではなく、平衡木の中の正しい位置に O(log n) で置くだけ
（`Vec::insert` + `sort` は 1 回 O(N) なので全体 O(N^2) になる）。並ぶのは**キーだけ**で、
`BTreeMap` の値は順序に関与しない。

## HashMap

| 操作 | 書き方 |
| --- | --- |
| カウントアップ | `*cnt.entry(x).or_insert(0) += 1;` |
| `Vec` に push | `g.entry(k).or_default().push(v);` |
| 無ければ 0 で取得 | `cnt.get(&x).copied().unwrap_or(0)` |
| 存在判定 | `cnt.contains_key(&x)` |
| 走査 | `for (k, v) in &cnt { }`（**順序不定**） |

### 頻度カウント → 同値ペアの個数

```rust
fn same_pairs(a: &[i64]) -> u64 {
    let mut cnt: HashMap<i64, u64> = HashMap::new();
    for &x in a {
        *cnt.entry(x).or_insert(0) += 1;
    }
    cnt.values().map(|&c| c * (c - 1) / 2).sum()
}
```

二重ループ O(N^2) を O(N) に落とす一番素直な使い方（`abc/abc473/src/bin/b.rs:14` が同じ形）。

### 累積和 × HashMap で「和が K の区間」を数える

```rust
fn count_subarrays_sum_k(a: &[i64], k: i64) -> u64 {
    let mut seen: HashMap<i64, u64> = HashMap::new();
    seen.insert(0, 1);                      // 空 prefix を先に 1 個入れておく
    let (mut sum, mut ans) = (0i64, 0u64);
    for &x in a {
        sum += x;
        ans += seen.get(&(sum - k)).copied().unwrap_or(0);
        *seen.entry(sum).or_insert(0) += 1;
    }
    ans
}
```

`sum[j] - sum[i] == k` を「今までに `sum - k` が何回出たか」に読み替える定石。区間の全探索
O(N^2) が O(N) になる。⚠️ `seen.insert(0, 1)` を忘れると先頭から始まる区間を数え落とす。

### `or_default()` でグルーピング

```rust
let mut g: HashMap<Vec<char>, Vec<String>> = HashMap::new();
for w in words {
    let mut key: Vec<char> = w.chars().collect();
    key.sort_unstable();
    g.entry(key).or_default().push(w.clone());   // Vec なら or_insert(vec![]) より短い
}
```

## HashSet

### `insert` の戻り値で訪問済み判定を 1 行に

```rust
let mut visited: HashSet<(usize, usize, u32)> = HashSet::new();
let mut q = VecDeque::new();
visited.insert(start);
q.push_back(start);
while let Some(s) = q.pop_front() {
    for nx in next_states(s) {
        if visited.insert(nx) {      // 新規なら true → そのまま push
            q.push_back(nx);
        }
    }
}
```

`contains` してから `insert` する必要はない。状態が `(座標, 持ち物ビットマスク)` のように
配列にしづらいときの BFS 用。状態が `0..N` の整数なら `vec![false; n]` の方が速い。

### 周期検出（ロー型のループ）

```rust
fn first_repeat(start: u64, n: u64) -> (u64, u64) {    // (ループ開始時刻, 周期長)
    let mut seen: HashMap<u64, u64> = HashMap::new();  // 状態 -> 初めて出た時刻
    let (mut x, mut t) = (start, 0u64);
    loop {
        if let Some(&prev) = seen.get(&x) {
            return (prev, t - prev);
        }
        seen.insert(x, t);
        x = next(x);
        t += 1;
    }
}
```

「K 回操作した後の状態を求めよ、K <= 10^18」の典型。時刻も欲しいので Set ではなく Map にする。

### 集合演算

```rust
let sa: HashSet<i64> = a.iter().copied().collect();
let sb: HashSet<i64> = b.iter().copied().collect();
sa.intersection(&sb).count();   // 共通
sa.difference(&sb).count();     // a にだけある
sa.union(&sb).count();          // 和集合
```

## BTreeMap

| 操作 | 書き方 |
| --- | --- |
| 区間を舐める | `for (k, v) in map.range(l..=r) { }` |
| lower_bound（x 以上で最小） | `map.range(x..).next()` |
| x 未満で最大 | `map.range(..x).next_back()` |
| 最小 / 最大 | `map.first_key_value()` / `map.last_key_value()` |

Rust には `lower_bound` メソッドが無いので `range(x..).next()` がその代わり。
ソート済み `Vec` + `binary_search` でも同じことができるが、**途中で挿入が入るなら BTree 一択**。

### multiset（削除できる多重集合）

```rust
struct MultiSet {
    map: BTreeMap<i64, usize>,
    len: usize,
}

impl MultiSet {
    fn new() -> Self {
        MultiSet { map: BTreeMap::new(), len: 0 }
    }
    fn insert(&mut self, x: i64) {
        *self.map.entry(x).or_insert(0) += 1;
        self.len += 1;
    }
    fn remove(&mut self, x: i64) -> bool {
        let Some(c) = self.map.get_mut(&x) else { return false };
        *c -= 1;
        if *c == 0 {
            self.map.remove(&x);     // 0 を残すと min/max が壊れる
        }
        self.len -= 1;
        true
    }
    fn min(&self) -> Option<i64> { self.map.first_key_value().map(|(&k, _)| k) }
    fn max(&self) -> Option<i64> { self.map.last_key_value().map(|(&k, _)| k) }
}
```

「集合に追加/削除しながら最小値を聞かれる」クエリ問題用。`BinaryHeap` は任意の値を削除できない
のでこちらになる。⚠️ カウントが 0 になったらキーごと消すこと。

### 最も近い値

```rust
fn nearest(map: &BTreeMap<i64, usize>, x: i64) -> Option<i64> {
    let hi = map.range(x..).next().map(|(&k, _)| k);        // x 以上で最小
    let lo = map.range(..x).next_back().map(|(&k, _)| k);   // x 未満で最大
    match (lo, hi) {
        (Some(l), Some(h)) => Some(if x - l <= h - x { l } else { h }),
        (l, h) => l.or(h),
    }
}
```

### 疎ないもす法（座標が 10^9 まであるとき）

```rust
fn max_overlap(events: &[(i64, i64)]) -> i64 {
    let mut diff: BTreeMap<i64, i64> = BTreeMap::new();
    for &(l, r) in events {
        *diff.entry(l).or_insert(0) += 1;
        *diff.entry(r).or_insert(0) -= 1;
    }
    let (mut cur, mut best) = (0, 0);
    for d in diff.values() {
        cur += d;
        best = best.max(cur);
    }
    best
}
```

配列でいもす法をやると長さ 10^9 で MLE。BTreeMap ならイベント点だけを持てて O(N log N)、
しかも走査が勝手に座標昇順になる。`HashMap` だと最後にソートが要る。ここが BTree を選ぶ理由。

## BTreeSet

| 操作 | 書き方 |
| --- | --- |
| 最小 / 最大 | `set.first()` / `set.last()` |
| 取り出して削除 | `set.pop_first()` / `set.pop_last()` |
| x 以上で最小 | `set.range(x..).next()` |
| x 未満で最大 | `set.range(..x).next_back()` |
| 上位 3 件 | `set.iter().rev().take(3)` |

### 「まだ処理していない添字」を飛ばして走る

```rust
fn skip_used(n: usize, ops: &[(usize, usize)]) -> Vec<usize> {
    let mut color = vec![0usize; n];
    let mut unused: BTreeSet<usize> = (0..n).collect();
    for &(l, c) in ops {
        let targets: Vec<usize> = unused.range(l..n).copied().collect();
        for i in targets {           // 走査中に remove できないので一度 Vec に出す
            color[i] = c;
            unused.remove(&i);
        }
    }
    color
}
```

「区間 [l, n) をまだ塗っていないマスだけ塗る」系。各要素は**高々 1 回しか触られない**ので、
クエリ数 Q・要素数 N に対して全体 O((N + Q) log N)。愚直に毎回区間を舐めると O(NQ) で落ちる。
この「消しながら range で次を引く」パターンが BTreeSet の一番の稼ぎどころ。

### 上位 K 件だけ保持して K 番目を逐次出力

```rust
fn kth_largest_online(a: &[usize], k: usize) -> Vec<usize> {
    let mut set: BTreeSet<(usize, usize)> = BTreeSet::new();  // (値, 添字) で多重集合化
    let mut out = vec![];
    for (i, &x) in a.iter().enumerate() {
        set.insert((x, i));
        if set.len() > k {
            set.pop_first();                 // 小さい方を捨てる
        }
        if i + 1 >= k {
            out.push(set.first().unwrap().0);
        }
    }
    out
}
```

`set` が常に K 要素なので、その最小がそのまま「降順 K 番目」。
⚠️ `BTreeSet` は集合なので同値が潰れる。添字が相異なることを使って `(値, 添字)` で入れると
多重集合になる。タプルの `Ord` は辞書式なので順序は第 1 要素で決まる
（詳細は `abc/abc476/src/bin/c.rs:20` のコメント）。

## BinaryHeap

**デフォルトは最大ヒープ**。最小ヒープが欲しければ `Reverse` で包む。

```rust
let mut h = BinaryHeap::new();
h.push(x); h.peek(); h.pop();            // 最大

let mut h = BinaryHeap::new();
h.push(Reverse((d, v)));                 // 最小ヒープ
while let Some(Reverse((d, v))) = h.pop() { }
```

### ダイクストラ

```rust
fn dijkstra(g: &[Vec<(usize, u64)>], s: usize) -> Vec<u64> {
    let mut dist = vec![u64::MAX; g.len()];
    let mut heap = BinaryHeap::new();
    dist[s] = 0;
    heap.push(Reverse((0u64, s)));                  // Reverse で最小ヒープ
    while let Some(Reverse((d, v))) = heap.pop() {
        if d > dist[v] {
            continue;                               // 古いエントリを捨てる
        }
        for &(to, w) in &g[v] {
            let nd = d + w;
            if nd < dist[to] {
                dist[to] = nd;
                heap.push(Reverse((nd, to)));       // 更新せず「積み直す」
            }
        }
    }
    dist
}
```

`BinaryHeap` は中の要素を書き換えられないので、**更新の代わりに新しいエントリを push して、
古いのは pop 時に捨てる**。⚠️ この 2 行（`Reverse` と `if d > dist[v]`）を落とすのが
ダイクストラのバグの大半。タプルは `(距離, 頂点)` の順にしないと距離で比較されない。

### 貪欲マージ（ハフマン / スライムの合成）

```rust
fn huffman(a: &[u64]) -> u64 {
    let mut heap: BinaryHeap<Reverse<u64>> = a.iter().map(|&x| Reverse(x)).collect();
    let mut total = 0;
    while heap.len() >= 2 {
        let Reverse(x) = heap.pop().unwrap();
        let Reverse(y) = heap.pop().unwrap();
        total += x + y;
        heap.push(Reverse(x + y));      // 合体した結果を戻す
    }
    total
}
```

「小さい 2 つをくっつけてコストを払う、を繰り返して最小化」。取り出した結果をまた入れるので、
ソート済み `Vec` では代用できない。ここが `BinaryHeap` の必然性。

### ヒープ 2 本で中央値を逐次取得

```rust
struct Median {
    lo: BinaryHeap<i64>,          // 下半分（最大ヒープ）
    hi: BinaryHeap<Reverse<i64>>, // 上半分（最小ヒープ）
}

impl Median {
    fn new() -> Self {
        Median { lo: BinaryHeap::new(), hi: BinaryHeap::new() }
    }
    fn push(&mut self, x: i64) {
        if self.lo.peek().map_or(true, |&t| x <= t) {
            self.lo.push(x);
        } else {
            self.hi.push(Reverse(x));
        }
        // lo.len() == hi.len() または hi.len() + 1 に均す
        if self.lo.len() > self.hi.len() + 1 {
            let t = self.lo.pop().unwrap();
            self.hi.push(Reverse(t));
        } else if self.hi.len() > self.lo.len() {
            let Reverse(t) = self.hi.pop().unwrap();
            self.lo.push(t);
        }
    }
    fn get(&self) -> Option<i64> { self.lo.peek().copied() }   // 下側中央値
}
```

`[5, 1, 4, 2, 9]` を順に入れると `5, 1, 4, 2, 4` を返す。「要素を追加しながら毎回中央値を
出力」に O(log N)/クエリで答える。

## ハマりどころ

- **`HashMap` / `HashSet` の走査順は実行ごとに変わる**（SipHash + ランダムシード）。出力順に
  使うと非決定的な WA になる。順序が要るなら BTree 系か、取り出して `sort`
- **標準ハッシュは競プロ的には遅い**。10^6 要素級で TLE 気味なら `rustc_hash` に差し替える（下記）
- **`f64` は `Ord` を実装しない**ので BTree / BinaryHeap に直接入らない。整数化するか
  `total_cmp` でラップする
- **`BinaryHeap` はデフォルト最大ヒープ**。`Reverse` を忘れると謎の答えが出る
- **タプルの `Ord` は辞書順**。`(距離, 頂点)` の順序を逆に書くと壊れる
- **`BTreeSet` を走査しながら `remove` できない**。一度 `Vec` に集めてから消す
- **標準に multiset は無い**。`BTreeMap<T, usize>` か `BTreeSet<(T, idx)>`、またはクレート

## クレートで済ませる

ジャッジで使える（`compete.toml` のテンプレート依存に含まれる）。動作確認済み。

| クレート | 用途 | 使い方 |
| --- | --- | --- |
| `rustc-hash` | 高速ハッシュ | `use rustc_hash::{FxHashMap, FxHashSet};` → `FxHashMap::default()` |
| `hashbag` | 多重集合（順序なし） | `HashBag::new()` / `insert` / `remove` / `contains` は**個数**を返す |
| `btreemultimap` | 1 キーに複数値（順序あり） | `BTreeMultiMap::new()` / `insert` / `get_vec(&k) -> Option<&Vec<V>>` |
| `counter` | 頻度集計 | `let c: Counter<char> = s.chars().collect();` / `c.most_common_ordered()` |

```rust
use rustc_hash::FxHashMap;

let mut m: FxHashMap<i64, usize> = FxHashMap::default();   // ::new() ではなく ::default()
*m.entry(3).or_insert(0) += 1;
```

`FxHashMap` は `HashMap` のハッシャ違いなので API は同一。標準より数倍速く、
`HashMap` が原因の TLE はこれで直ることが多い（ただしハッシュが単純なので、
意図的な hack のある問題では避ける）。

## 参考

- [std::collections](https://doc.rust-lang.org/std/collections/) — 冒頭に選択ガイドがある
- 実例: `abc/abc476/src/bin/c.rs`（BTreeSet の多重集合化 + BinaryHeap の別解）
