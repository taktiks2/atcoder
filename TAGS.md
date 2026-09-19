# アルゴリズムタグ語彙集

各問題のソース先頭に付ける `// algo:` 行で使うタグの一覧。

- 書式: `// algo: tag1, tag2`（**ファイルの 1 行目**に置く、カンマ区切り）
- 初出のタグは **先にここへ追記してから** 使う（表記ゆれ防止）
- 迷ったら細かい方を付ける。部分一致検索なので `bfs` で `multi-source-bfs` もヒットする
- 検索: `just algo <tag>` / 使用回数の集計: `just algos`

## 別解メモの規約 (`// alt:`)

AC コードとは別の解法・書き方をコメントで残すときのルール。

- マーカーは `// alt:` で固定し、1 行目に **何がどう良いか**を必ず書く（観点: Rust らしさ / 計算量 / 短さ など）。「より良い解法」だけでは後から学びが分からない
- 置き場所は **main の後ろ（ファイル末尾）**。AC コードは常に main に置き、提出対象を構造で分かるようにする
- 1 別解 = 1 ブロック。複数あれば `// alt:` ブロックを並べる
- 「絶対に動く形で残したい」別解はコメントではなく `#[allow(dead_code)]` 関数にする（型チェックが効き続け、腐らない）
- 検索: `grep -rn '^// alt:' abc books other` で別解メモを残した問題を一覧できる

```rust
// algo: string
use proconio::input;

fn main() {
    // AC したコード
}

// alt: replace で宣言的に書ける・最短
// input! { s: String }
// println!("{}", s.replace(|c| c != 'A', "."));
```

## 探索

| タグ | 意味 | 使わない表記 |
| --- | --- | --- |
| brute-force | 全探索 | full-search, zentansaku |
| enumeration | 列挙（候補を直接構成して並べる） | |
| bfs | 幅優先探索 | 幅優先 |
| multi-source-bfs | 多始点 BFS | マルチソースBFS |
| 01-bfs | 0-1 BFS | zero-one-bfs |
| dfs | 深さ優先探索 | 深さ優先 |
| binary-search | 二分探索 | nibutan |

## 累積・区間

| タグ | 意味 | 使わない表記 |
| --- | --- | --- |
| prefix-sum | 累積和 | cumulative-sum, ruisekiwa |
| 2d-prefix-sum | 二次元累積和 | |
| imos | いもす法 | |
| sliding-window | スライディングウィンドウ | |
| two-pointers | 尺取り法 | shakutori |

## DP

| タグ | 意味 | 使わない表記 |
| --- | --- | --- |
| dp | 動的計画法（全般） | dynamic-programming |
| knapsack | ナップサック DP | |
| bit-dp | ビット DP | bitdp |
| tree-dp | 木 DP | |

## グラフ

| タグ | 意味 | 使わない表記 |
| --- | --- | --- |
| graph | グラフ（全般） | adjacency-list |
| connected-components | 連結成分の数え上げ・分解 | cc |
| tree | 木 | |
| dijkstra | ダイクストラ法 | |
| bellman-ford | ベルマンフォード法 | |
| floyd-warshall | ワーシャルフロイド法 | warshall-floyd |
| topological-sort | トポロジカルソート | toposo |

## データ構造

| タグ | 意味 | 使わない表記 |
| --- | --- | --- |
| union-find | Union-Find | dsu |
| priority-queue | 優先度付きキュー | heap, binary-heap |
| segment-tree | セグメント木 | segtree |
| fenwick-tree | BIT（Binary Indexed Tree） | bit |
| deque | 両端キュー（VecDeque） | |
| set | 集合で既出判定・重複除去 | hashset |

## 数学

| タグ | 意味 | 使わない表記 |
| --- | --- | --- |
| math | 数学（整数論・組合せ全般） | |
| gcd | 最大公約数・最小公倍数 | lcm |
| combinatorics | 組合せ・場合の数 | |
| sieve | エラトステネスの篩・素数列挙 | prime-sieve |
| modint | mod 演算・繰り返し二乗法 | mod-pow, mod-arithmetic |
| parity | 偶奇に着目する | |
| radix-conversion | 基数変換 | |

## 文字列

| タグ | 意味 | 使わない表記 |
| --- | --- | --- |
| string | 文字列処理（全般） | |
| run-length | ランレングス圧縮 | rle |

## その他

| タグ | 意味 | 使わない表記 |
| --- | --- | --- |
| greedy | 貪欲法 | |
| exchange-argument | 交換論法（貪欲の正当性） | |
| constructive | 構築 | |
| counting | 出現回数のカウント・度数集計 | count, frequency |
| sort | ソートして考える | sorting |
| simulation | シミュレーション・愚直 | |
| grid | グリッド問題 | |
| coordinate-compression | 座標圧縮 | zaatsu |
| game | ゲーム・勝敗判定 | |
| interactive | インタラクティブ | |
