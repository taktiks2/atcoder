// algo: graph, adjacency-list
use proconio::{input, marker::Usize1};

// 各頂点 i について「隣接頂点のうち番号が i より小さいもの」の個数を数え、それが 1 の
// 頂点数を返す。無向辺 (a, b) を両方向に登録した隣接リスト上で全頂点 × 各隣接頂点を
// 走査するので、全体 O(N + M) で回る。N, M <= 10^5 なので余裕。

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    };

    let mut graph = vec![vec![]; n];

    for (a, b) in ab {
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut cnt = 0;
    for (i, neighbors) in graph.iter().enumerate() {
        let count = neighbors.iter().filter(|&&x| x < i).count();
        if count == 1 {
            cnt += 1;
        }
    }

    println!("{cnt}");
}

// alt: 隣接リストを作らず、辺 (a, b) を読むたびに smaller[max(a, b)] を +1 するだけで
//      済む。max(a, b) から見て min(a, b) は必ず「自分より小さい隣接頂点」で、辺 1 本
//      ごとに 1 頂点だけ寄与するため、辺を 1 パスなめれば各頂点の該当カウントが確定する。
//      メモリ O(N)、時間 O(N + M) で隣接リスト実装より軽い
// let mut smaller = vec![0usize; n];
// for (a, b) in ab {
//     smaller[a.max(b)] += 1;
// }
// let cnt = smaller.iter().filter(|&&c| c == 1).count();

// alt: filter+count を使わず素直な for で数える書き方。イテレータチェーンに慣れる前や、
//      「小さい隣接頂点を数えつつ別条件も一緒に判定する」等の拡張を入れたい場合は
//      こちらの方が読みやすい
// let mut cnt = 0;
// for (i, neighbors) in graph.iter().enumerate() {
//     let mut c = 0;
//     for &x in neighbors {
//         if x < i {
//             c += 1;
//         }
//     }
//     if c == 1 { cnt += 1; }
// }
