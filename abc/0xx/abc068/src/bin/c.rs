// algo: graph, bfs
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 島 1 から島 N へ、定期便をちょうど 2 本乗り継いで行けるかを判定する問題。
// 制約で (1, N) の直行便は無いと保証されているので、「2 本以内で行けるか」と
// 「ちょうど 2 本で行けるか」は同じ意味になる。つまり 1 を始点にした最短距離が
// dist[N] == 2 になるかを見ればよい。
//
// 素朴には便を 2 本選んで全ペアを試すことになるが、M = 2×10^5 だと M^2 = 4×10^10 で TLE。
// BFS なら頂点も辺も 1 回ずつしか触らないので O(N + M) = 4×10^5 程度で済む。
//
// 判定を「発見した瞬間」にしてよいのは、BFS では各頂点が最短距離で一度だけ発見される
// から。N を初めて見つけたときの next がそのまま dist[N] になる。break で抜けるのは
// 内側の for だけで while は回り続けるが、BFS 全体が O(N + M) なので答えにも計算量にも
// 影響しない (早期終了にはなっていない、というだけ)。
//
// Usize1 で 0-indexed にしているので、島 N は添字 n - 1 になる。判定を j == n や
// dist[j] == n と書くと絶対に true にならず、全入力で IMPOSSIBLE が出る。サンプル 2・3
// は答えが IMPOSSIBLE なので通ってしまい、気づけるのは 1・4 だけ。
//
// 辺は双方向に張っているが、この問題では a_i < b_i なので片方向 (a -> b) でも答えは
// 変わらない。2 本の経路は 1 -> v -> N の形しか無く、1 < v < N なので 2 本とも
// 「番号の小さい側から大きい側へ」向かう。abc277 c のように一度下る経路が要る問題とは
// ここが違う。
//
// 型は usize。距離は高々 n なので、dist[i] + 1 の i は必ず訪問済みで INF にならず、
// オーバーフローしない。let INF は non_snake_case の警告が出るので、定数として持つなら
// const INF: usize = usize::MAX; が Rust の慣習。
// 出力は 1 行なので #[fastout] は本問では効いていない。

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    };

    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];

    for (ai, bi) in ab {
        graph[ai].push(bi);
        graph[bi].push(ai);
    }

    let INF = usize::MAX;
    let mut queue: VecDeque<usize> = VecDeque::from([0]);
    let mut dist: Vec<usize> = vec![INF; n];
    dist[0] = 0;

    let mut ans = false;

    while let Some(i) = queue.pop_front() {
        for &j in &graph[i] {
            if dist[j] == INF {
                let next = dist[i] + 1;
                if j + 1 == n && next == 2 {
                    ans = true;
                    break;
                }
                dist[j] = next;
                queue.push_back(j);
            }
        }
    }

    println!("{}", if ans { "POSSIBLE" } else { "IMPOSSIBLE" });
}

// alt: 距離が 2 かどうかしか問われていないので、BFS すら要らない。「1 から直接行ける島」と
//      「N へ直接行ける島」の両方に入っている島 v があれば答えは POSSIBLE。a_i < b_i なので
//      1 に接する辺は必ず a == 1、N に接する辺は必ず b == N の形で来て、片側だけ見れば済む。
//      グラフもキューも作らず O(N + M)。最短で、問題の構造がそのまま出る
// let mut from1 = vec![false; n];
// let mut to_n = vec![false; n];
// for &(a, b) in &ab {
//     if a == 0 {
//         from1[b] = true;
//     }
//     if b == n - 1 {
//         to_n[a] = true;
//     }
// }
// let ok = (0..n).any(|v| from1[v] && to_n[v]);
// println!("{}", if ok { "POSSIBLE" } else { "IMPOSSIBLE" });

// alt: BFS のまま書くなら、ループの中で判定せず、最後まで回してから dist[n - 1] == 2 を
//      見る方が素直。ans フラグも break も消え、「最短距離が 2 か」という条件がそのまま
//      コードになる。発見時に判定する形で起きやすい、添字や条件の書き間違いも減る
// let mut graph = vec![vec![]; n];
// for &(a, b) in &ab {
//     graph[a].push(b);
//     graph[b].push(a);
// }
// let mut dist = vec![usize::MAX; n];
// dist[0] = 0;
// let mut queue = VecDeque::from([0]);
// while let Some(v) = queue.pop_front() {
//     for &u in &graph[v] {
//         if dist[u] == usize::MAX {
//             dist[u] = dist[v] + 1;
//             queue.push_back(u);
//         }
//     }
// }
// println!("{}", if dist[n - 1] == 2 { "POSSIBLE" } else { "IMPOSSIBLE" });
