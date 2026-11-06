// algo: graph, bfs
// solve: self 2026-11-06
#![allow(unused_imports)]
use indexing::algorithms::binary_search;
use itertools::Itertools;
use pathfinding::{directed::bfs, prelude::bfs};
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 単純無向グラフがパスグラフ (一本道) か判定する問題。N, M <= 2×10^5。
// 頂点の並べ方 N! 通りを試すのは論外なので、「一本道の長さ」を測って判定する。
//
// 判定の核心は「ある頂点からの BFS 距離の最大値が N-1 になる ⇔ パスグラフ」。
// (⇐) パスグラフなら端点から反対の端点まで距離 N-1。
// (⇒) 距離 N-1 の最短路は辺 N-1 本・頂点 N 個を重複なく通るので、全頂点を一列に並べた
//     ものになる。そこに列で隣り合わない 2 頂点を結ぶ辺が 1 本でもあれば近道ができて
//     最短距離が N-1 未満に縮むので、余分な辺は存在しない。よってパスグラフ。
// この (⇒) はどの頂点から BFS しても成り立つので、閉路があって「二回 BFS で直径」が
// 正確でないグラフでも誤って Yes を出すことはない。非連結なら届かない頂点が -1 のまま
// 残り、最大値は N-1 に届かない (サンプル 2 は dist = [0, -1] で 0+1 != 2 -> No)。
//
// BFS を二回やるのは始点を端点にするため。頂点 0 が列の途中にあると一回目の最大値は
// N-1 未満になる (例: 2-1-3 で頂点 1 から測ると最大 1)。パスグラフなら頂点 0 から最も
// 遠い頂点は必ず端点なので、そこからもう一度測れば N-1 が出る。木の直径を求める
// double sweep と同じ形。一回だけの BFS でもサンプル 1 は頂点 1 がたまたま端点
// (1-3-2-4) なので通ってしまう。「サンプルは通るのに WA」の形。
//
// 「M == N-1 かつ全頂点の次数 <= 2」だけで判定するのも罠。三角形 + 孤立点
// (N=4, M=3) は両方満たすが No。連結性の確認が要る (下の alt 参照)。
//
// position_max は最大値が複数あると最後のものを返すが、パスグラフなら最大距離を
// 取る頂点はどれも端点なので問題ない。dist を i64 にしているのは未訪問の番兵 -1 の
// ため (usize なら usize::MAX を番兵にする手もあるが、max を取ると番兵が勝ってしまう)。
// 計算量は BFS 二回で O(N+M)。出力は 1 行なので #[fastout] は本問では効いていない。

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
    };

    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for (u, v) in uv {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut dist = vec![-1i64; n];
    let mut queue = VecDeque::new();
    dist[0] = 0;
    queue.push_back(0);
    while let Some(v) = queue.pop_front() {
        for &to in &graph[v] {
            if dist[to] == -1 {
                dist[to] = dist[v] + 1;
                queue.push_back(to);
            }
        }
    }

    let tmp = dist.iter().position_max().unwrap();

    let mut dist = vec![-1i64; n];
    let mut queue = VecDeque::new();
    dist[tmp] = 0;
    queue.push_back(tmp);
    while let Some(v) = queue.pop_front() {
        for &to in &graph[v] {
            if dist[to] == -1 {
                dist[to] = dist[v] + 1;
                queue.push_back(to);
            }
        }
    }

    let ans = dist.into_iter().max().unwrap();

    println!("{}", if (ans + 1) == n as i64 { "Yes" } else { "No" });
}

// alt: リファクタリング版。構造はそのまま (BFS 二回で最遠点 -> 最大距離)
//      - 同じ BFS を二回コピペしていたのをクロージャ bfs(s) にまとめた
//      - 初期化 + push_back を VecDeque::from([s]) の 1 行に
//      - 一時変数 tmp を far に改名し、dist の再宣言 (シャドーイング) を消した
// let bfs = |s: usize| -> Vec<i64> {
//     let mut dist = vec![-1i64; n];
//     let mut queue = VecDeque::from([s]);
//     dist[s] = 0;
//     while let Some(v) = queue.pop_front() {
//         for &to in &graph[v] {
//             if dist[to] == -1 {
//                 dist[to] = dist[v] + 1;
//                 queue.push_back(to);
//             }
//         }
//     }
//     dist
// };
//
// let far = bfs(0).into_iter().position_max().unwrap();
// let ans = bfs(far).into_iter().max().unwrap();
// println!("{}", if ans + 1 == n as i64 { "Yes" } else { "No" });

// alt: パスグラフの特徴付け「辺 N-1 本・次数 <= 2・連結」を直接チェックする版。
//      距離の議論が要らず、条件がそのまま読めるので短い。辺 N-1 本で連結なら木、
//      次数 <= 2 の木は一本道。連結性を落とすと三角形 + 孤立点で WA になる
// use ac_library::Dsu;
// let mut deg = vec![0; n];
// let mut dsu = Dsu::new(n);
// for &(u, v) in &uv {
//     deg[u] += 1;
//     deg[v] += 1;
//     dsu.merge(u, v);
// }
// let ok = m == n - 1 && deg.iter().all(|&d| d <= 2) && dsu.size(0) == n;
// println!("{}", if ok { "Yes" } else { "No" });
