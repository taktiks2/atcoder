// algo: bfs, graph
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 各ユーザーについて「友達の友達」の人数を出す問題。友達の友達とは、自分でも直接の友達でも
// なく、共通の友達がいる人のこと。グラフで言えば、i からの最短距離がちょうど 2 の頂点。
// 距離 0 (自分) と距離 1 (直接の友達) は、BFS の距離で自然に弾かれる。
//
// N <= 10 なので、全員を始点に BFS を N 回回しても O(N(N + M)) で、計算量は気にしなくてよい。
//
// 落とし穴は非連結の場合。M = 0 も制約で許されているので、i から届かない頂点の dist は
// None のまま残る。出力で x.unwrap() == 2 と書くと、そこで panic して RE になる。
// サンプル 3 つはどれもグラフがつながっているので全部通ってしまう、「サンプルは通るのに RE」
// の典型形。Option のまま x == Some(2) と比べれば、None は false になるだけで済む。
//
// BFS 内の dist[start] は、キューに積む直前に必ず Some を入れているので None にならない。
// unwrap でも落ちないが、map で「親の距離 + 1」をそのまま書けば unwrap を置く必要すらない。
// unwrap_or(0) のようにダミー値を入れる書き方は、前提が崩れたときに間違った距離が黙って入る
// ので避ける。

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

    for i in 0..n {
        let mut queue: VecDeque<usize> = VecDeque::from([i]);
        let mut dist: Vec<Option<usize>> = vec![None; n];
        dist[i] = Some(0);

        while let Some(start) = queue.pop_front() {
            for &to in &graphs[start] {
                if dist[to].is_none() {
                    dist[to] = dist[start].map(|d| d + 1);
                    queue.push_back(to);
                }
            }
        }

        println!("{}", dist.iter().filter(|&&x| x == Some(2)).count());
    }
}

// alt: キューに (頂点, 距離) を積めば、dist[start] を読み返さないので Option の出し入れが
//      消える。dist は訪問済みフラグ兼、出力用の距離としてだけ使う
// for i in 0..n {
//     let mut dist: Vec<Option<usize>> = vec![None; n];
//     dist[i] = Some(0);
//     let mut queue = VecDeque::from([(i, 0)]);
//     while let Some((v, d)) = queue.pop_front() {
//         for &to in &graphs[v] {
//             if dist[to].is_none() {
//                 dist[to] = Some(d + 1);
//                 queue.push_back((to, d + 1));
//             }
//         }
//     }
//     println!("{}", dist.iter().filter(|&&x| x == Some(2)).count());
// }

// alt: 定義を直訳すると BFS が要らない。隣接行列にすれば「i と j が友達でなく、共通の友達 k
//      がいる」をそのまま式にできる。O(N^3) = 1000 で、N <= 10 なら何の問題もない。
//      非連結でも未訪問という状態が存在しないので、今回の RE の余地が構造的にない
// let mut adj = vec![vec![false; n]; n];
// for &(a, b) in &ab {
//     adj[a][b] = true;
//     adj[b][a] = true;
// }
// for i in 0..n {
//     let cnt = (0..n)
//         .filter(|&j| j != i && !adj[i][j] && (0..n).any(|k| adj[i][k] && adj[k][j]))
//         .count();
//     println!("{cnt}");
// }

// alt: 隣接リストを HashSet にし、友達の友達を集めてから「自分」と「直接の友達」を除く。
//      重複 (共通の友達が 2 人以上いる人) は HashSet が勝手に潰す。
//      「2 歩先の集合 − 1 歩先 − 自分」という集合演算として読める
// let mut graphs: Vec<HashSet<usize>> = vec![HashSet::new(); n];
// for (a, b) in ab {
//     graphs[a].insert(b);
//     graphs[b].insert(a);
// }
// for i in 0..n {
//     let fof: HashSet<usize> = graphs[i]
//         .iter()
//         .flat_map(|&k| graphs[k].iter().copied())
//         .filter(|&j| j != i && !graphs[i].contains(&j))
//         .collect();
//     println!("{}", fof.len());
// }
