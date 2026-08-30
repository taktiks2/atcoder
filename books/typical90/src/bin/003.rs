// algo: tree, bfs
use proconio::{input, marker::Usize1};
use std::collections::VecDeque;

// 木の直径 + 1
//
// 木は閉路を持たないので、u-v 間に辺を 1 本足すとできる閉路はただ 1 つ。
// その長さは「木上の u→v パスの辺数 + 1」なので、最大化したいのは木上の最長パス = 直径。
//
// 直径は BFS 2 回で取れる。「任意の点からの最遠点は必ず直径の端点になる」ので、
// 1 回目は端点を 1 つ拾うだけ、実際の測定は 2 回目が担当する。
// (端点でない点 t が最遠だとすると、直径パスとの合流点 m を経由して直径より長いパスが作れて矛盾)

const INF: usize = usize::MAX;

fn bfs(graph: &[Vec<usize>], start: usize) -> Vec<usize> {
    let mut dist = vec![INF; graph.len()];

    let mut queue = VecDeque::from([start]);
    dist[start] = 0usize;

    while let Some(x) = queue.pop_front() {
        for &n in &graph[x] {
            if dist[n] == INF {
                queue.push_back(n);
                dist[n] = dist[x] + 1;
            }
        }
    }

    dist
}

fn main() {
    input! {
        n: usize,
        roads: [(Usize1, Usize1); n - 1],
    };

    let mut graph = vec![vec![]; n];

    for (a, b) in roads {
        graph[a].push(b);
        graph[b].push(a);
    }

    let dist = bfs(&graph, 0);

    let (end_point, _) = dist.iter().enumerate().max_by_key(|&(_, v)| v).unwrap();

    let dist = bfs(&graph, end_point);

    let ans = dist.iter().max().unwrap() + 1;
    println!("{ans}");
}

// alt: bfs が (最遠点, その距離) まで返すと、呼び出し側から max_by_key が消えて main が 3 行になる。
//      「距離配列が欲しい」のではなく「最遠点が欲しい」という意図がシグネチャに出る
// fn farthest(graph: &[Vec<usize>], start: usize) -> (usize, usize) {
//     let dist = bfs(graph, start);
//     dist.iter().enumerate().max_by_key(|&(_, &d)| d).map(|(i, &d)| (i, d)).unwrap()
// }
// let (end_point, _) = farthest(&graph, 0);
// let (_, diameter) = farthest(&graph, end_point);
// println!("{}", diameter + 1);

// alt: 木 DP。走査 1 回で直径が出る。各頂点を折り返し地点とする最長パスが副産物で手に入るので、
//      部分木ごとの情報が要る問題に流用しやすい (代わりに親の管理が要るぶん実装量は増える)
//      BFS で作った訪問順を逆にたどれば、再帰なしで葉から畳み込める
// let (mut order, mut parent) = (Vec::with_capacity(n), vec![INF; n]);
// let mut queue = VecDeque::from([0usize]);
// parent[0] = 0;
// while let Some(x) = queue.pop_front() {
//     order.push(x);
//     for &c in &graph[x] {
//         if parent[c] == INF {
//             parent[c] = x;
//             queue.push_back(c);
//         }
//     }
// }
//
// let mut down = vec![0usize; n]; // down[v] = v から下に伸びる最長パスの辺数
// let mut diameter = 0;
// for &v in order.iter().rev() {
//     let (mut a, mut b) = (0usize, 0usize); // 子方向の最長 2 本
//     for &c in &graph[v] {
//         if c == parent[v] {
//             continue;
//         }
//         let d = down[c] + 1;
//         if d > a {
//             b = a;
//             a = d;
//         } else if d > b {
//             b = d;
//         }
//     }
//     down[v] = a;
//     diameter = diameter.max(a + b); // v で折り返すパスの最大長
// }
// println!("{}", diameter + 1);
