// algo: graph, bfs, dp, modint
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 都市 1 から都市 N への最短経路が何通りあるかを mod 10^9+7 で数える問題。
// 経路を一本ずつ列挙するのは論外で、層が 40 段・各層 10 頂点のグラフ (約 400 頂点)
// でも最短経路は 10^40 通りある。数えるべきは経路そのものではなく「各頂点に何通りで
// 着くか」。
//
// 頂点 v への最短経路の最後の一歩は、必ず dist[u] + 1 == dist[v] を満たす隣接頂点 u
// から来る。よって cnt[v] = Σ cnt[u] (u はその条件を満たす隣接頂点)。この条件を満たす
// 辺だけに向きを付けて残すと、距離の層ごとに一方向へ流れる DAG になり、その上の経路
// 数え上げは DP になる。BFS はキューから距離の昇順に取り出すので、取り出し順がその
// まま DAG のトポロジカル順になる。v を取り出した時点で距離 dist[v] - 1 の頂点は全部
// 取り出し済みだから cnt[v] は確定していて、そこから隣へ配ってよい (配る DP)。BFS
// 一回の中で距離と通り数が同時に決まり、O(N + M) = 4×10^5 程度。
//
// match の 3 分岐がそのまま状態の場合分けになっている。
//   INF              : 初めて見つけた。to への最短経路の最初の候補が v 経由なので
//                      cnt[to] = cnt[v] (1 ではない)
//   dist[v] + 1      : 別の最短経路が見つかった。足し込む
//   それ以外         : 同じ層どうしの辺か、前の層へ戻る辺。最短経路には乗らないので無視
// 未訪問で cnt[to] = 1 と書くと、1 層目 (cnt が全部 1) しか正しくならない。sample1 は
// それでも通ってしまい、気づけるのは sample4 (正解 4 に対して 2 が出る) だけ。
//
// mod を足すたびに取ってよいのは (a + b) mod m = (a mod m + b mod m) mod m だから。
// 更新は足し算だけなので、途中で何回余りに置き換えても最後の余りは変わらない。
// INF 分岐で cnt[v] を mod なしで写しているのは、cnt[v] が既に mod 未満なので問題ない。
//
// 最初の提出は mod を 10 ^ 9 + 7 と書いて WA だった。Rust の ^ はべき乗ではなく XOR で、
// さらに + の方が優先順位が高いので 10 ^ (9 + 7) = 10 ^ 16 = 26 になる。つまり 26 で
// 割った余りを出していた。サンプルの答えは 2, 1, 0, 4 と全部 26 未満なので、4 つとも
// 通ってしまう。「サンプルは通るのに WA」の典型形で、定数は 1_000_000_007 と直に書く。
//
// 型は usize (64 bit)。cnt[v] + cnt[to] は高々 2×(10^9+7) なので溢れない
// (32 bit なら 4.29×10^9 に迫るので危うい)。dist[v] + 1 の v はキューから出た時点で
// 訪問済みなので INF にならず、これも溢れない。到達不能なら cnt[n - 1] が 0 のまま
// 残るので、特別扱いなしで「0 を出力」の仕様を満たす。
// 出力は 1 行なので #[fastout] は本問では効いていない。

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        roads: [(Usize1, Usize1); m],
    };

    let mut graphs: Vec<Vec<usize>> = vec![vec![]; n];

    for (ai, bi) in roads {
        graphs[ai].push(bi);
        graphs[bi].push(ai);
    }

    const INF: usize = usize::MAX;
    let mut queue: VecDeque<usize> = VecDeque::from([0]);
    let mut dist: Vec<usize> = vec![INF; n];
    let mut cnt: Vec<usize> = vec![0; n];
    dist[0] = 0;
    cnt[0] = 1;

    while let Some(v) = queue.pop_front() {
        for &to in &graphs[v] {
            match dist[to] {
                INF => {
                    dist[to] = dist[v] + 1;
                    cnt[to] = cnt[v];
                    queue.push_back(to);
                }
                d if d == dist[v] + 1 => {
                    cnt[to] = (cnt[v] + cnt[to]) % 1_000_000_007;
                }
                _ => {}
            }
        }
    }

    println!("{}", cnt[n - 1]);
}

// alt: BFS (距離を決める) と DP (通り数を数える) を二段に分けた「集める DP」版。
//      BFS の取り出し順 order を記録しておき、その順に cnt[v] = Σ cnt[u] を式どおり
//      計算する。「最短経路 DAG をトポロジカル順に DP する」という構造がコードの形に
//      そのまま出るので、Dijkstra + 通り数のように距離の決め方が変わっても DP 側は
//      使い回せる。sum は次数 × 10^9 ≲ 2×10^14 で usize に収まるので、mod は最後に一回で済む
// const MOD: usize = 1_000_000_007;
// let mut dist = vec![usize::MAX; n];
// let mut order = Vec::with_capacity(n);
// let mut queue = VecDeque::from([0]);
// dist[0] = 0;
// while let Some(v) = queue.pop_front() {
//     order.push(v);
//     for &to in &graphs[v] {
//         if dist[to] == usize::MAX {
//             dist[to] = dist[v] + 1;
//             queue.push_back(to);
//         }
//     }
// }
// let mut cnt = vec![0usize; n];
// cnt[0] = 1;
// for &v in &order[1..] {
//     cnt[v] = graphs[v]
//         .iter()
//         .filter(|&&u| dist[u] + 1 == dist[v])
//         .map(|&u| cnt[u])
//         .sum::<usize>()
//         % MOD;
// }
// println!("{}", cnt[n - 1]);

// alt: キューの代わりに層 (同じ距離の頂点の Vec) ごとに進める版。今の層を処理している
//      間は次の層に一切手を付けないので、「配る時点で cnt[v] が確定している」ことが
//      構造で保証される。さらに cnt の初期値 0 を活かして、初めて見つけた頂点も
//      if を抜けて足し込みに落とせば「未訪問なら cnt[v] を写す」と「足し込む」が一本の
//      式にまとまり、cnt[to] = 1 と書き間違える余地が消える
// const MOD: usize = 1_000_000_007;
// let mut dist = vec![usize::MAX; n];
// let mut cnt = vec![0usize; n];
// dist[0] = 0;
// cnt[0] = 1;
// let mut layer = vec![0];
// let mut d = 0;
// while !layer.is_empty() {
//     let mut next = vec![];
//     for &v in &layer {
//         for &to in &graphs[v] {
//             if dist[to] == usize::MAX {
//                 dist[to] = d + 1;
//                 next.push(to);
//             }
//             if dist[to] == d + 1 {
//                 cnt[to] = (cnt[to] + cnt[v]) % MOD;
//             }
//         }
//     }
//     layer = next;
//     d += 1;
// }
// println!("{}", cnt[n - 1]);
