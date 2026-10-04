// algo: graph, bfs
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 部屋 1 以外の各部屋に「隣の部屋」を 1 つ指す道しるべを置き、どこから辿っても最短で
// 部屋 1 に着くようにする問題。言い換えると、各部屋 v には「部屋 1 からの距離が
// dist[v] - 1 の隣の部屋」を書けばよい。そうすれば 1 歩ごとに距離がちょうど 1 減るので、
// dist[v] 歩で部屋 1 に着き、これが最小になる。逆に距離が減らない部屋を指すと最短には
// ならないので、条件は「距離が 1 小さい隣を指す」ことと同じ。
//
// 距離は部屋 1 を始点にした BFS で出る。BFS で to を x から初めて見つけたとき
// dist[to] = dist[x] + 1 が確定しているので、見つけた元の x がそのまま「距離が 1 小さい隣」
// になる。だから距離の配列は持たず、BFS 木の親 (x + 1 で 1-indexed に直した値) を marks に
// 入れるだけで答えになる。頂点も辺も 1 回ずつしか触らないので O(N + M) = 3×10^5 程度。
//
// DFS では駄目。DFS 木の親は最短経路上の頂点とは限らない。三角形 1-2, 2-3, 1-3 で 1 -> 2 -> 3
// と潜ると 3 の親が 2 になるが、3 は 1 と隣接していて距離は 1。サンプルが通っても落ちうる。
//
// 制約で「どの 2 部屋も行き来できる」(連結) と保証されているので、全部屋が BFS で見つかり、
// 答えは常に Yes。No の分岐は実際には通らない。「No になる場合を探す」問題ではない。
//
// marks[0] = 0 は訪問済みの印として置いている番兵で、部屋 1 を隣から再発見して親を
// 書き込むのを防ぐ。値そのものは skip(1) で出力しないので 0 でよい。
//
// 答えは一意ではない (距離が 1 小さい隣が複数あればどれでもよい)。隣接リストの順で親が
// 変わるので、別の書き方をすると cargo compete test の完全一致ではサンプル不一致になりうる
// が、正しい答えなら AC になる。
//
// 型は usize。marks に入る値は高々 N = 10^5 で溢れない。出力が最大 10^5 行あるので、
// println! ごとのフラッシュを避ける #[fastout] が本問ではちゃんと効いている。

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    };

    let mut graphs: Vec<Vec<usize>> = vec![vec![]; n];

    for (ai, bi) in ab {
        graphs[ai].push(bi);
        graphs[bi].push(ai);
    }

    const INF: usize = usize::MAX;
    let mut queue: VecDeque<usize> = VecDeque::from([0]);
    let mut marks: Vec<usize> = vec![INF; n];
    marks[0] = 0;

    while let Some(x) = queue.pop_front() {
        for &to in &graphs[x] {
            if marks[to] == INF {
                marks[to] = x + 1;
                queue.push_back(to);
            }
        }
    }

    if marks.iter().all(|&x| x != INF) {
        println!("Yes");
        for m in marks.iter().skip(1) {
            println!("{m}");
        }
    } else {
        println!("No");
    }
}

// alt: 番兵の usize::MAX を Option<usize> に置き換えた版。「未訪問」が None として型で表れ、
//      INF との比較を書き忘れる余地が無くなる。連結保証を使って No の分岐を消し、出力は
//      join で 1 回にまとめるので #[fastout] なしでも速い。Rust らしく、短い
// let mut graph = vec![vec![]; n];
// for &(a, b) in &ab {
//     graph[a].push(b);
//     graph[b].push(a);
// }
// let mut parent: Vec<Option<usize>> = vec![None; n];
// parent[0] = Some(0); // 番兵。部屋 1 を再訪しないためだけの値で、出力しない
// let mut queue = VecDeque::from([0]);
// while let Some(v) = queue.pop_front() {
//     for &u in &graph[v] {
//         if parent[u].is_none() {
//             parent[u] = Some(v);
//             queue.push_back(u);
//         }
//     }
// }
// println!("Yes\n{}", parent[1..].iter().map(|p| p.unwrap() + 1).join("\n"));

// alt: 普通に距離を求めてから、各部屋で「距離が 1 小さい隣」を後から探す版。BFS 木の親を
//      記録する工夫に頼らず、問題の条件 (dist[u] + 1 == dist[v]) がそのままコードになるので、
//      なぜ正しいかが読み取りやすい。隣を舐め直す分も合計 O(N + M) で計算量は変わらない
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
// println!("Yes");
// for v in 1..n {
//     let u = graph[v].iter().find(|&&u| dist[u] + 1 == dist[v]).unwrap();
//     println!("{}", u + 1);
// }
