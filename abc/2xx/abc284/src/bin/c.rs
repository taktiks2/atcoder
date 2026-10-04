// algo: graph, bfs, connected-components
use proconio::{input, marker::Usize1};
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1); m],
    };

    let mut graph = vec![vec![]; n];

    // 隣接リストの作成
    for (u, v) in edges {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut queue = VecDeque::new();

    let mut visited = vec![false; n];

    // 初期場所へ
    let mut ans = 0;

    for i in 0..n {
        if !visited[i] {
            ans += 1;
            queue.push_back(i);
            while let Some(base) = queue.pop_front() {
                for &x in &graph[base] {
                    if !visited[x] {
                        visited[x] = true;
                        queue.push_back(x);
                    }
                }
            }
        }
    }

    println!("{ans}");
}

// alt: ac-library の Dsu (Union-Find) なら visited・キューの管理が丸ごと消えて最短
// use ac_library::Dsu;
// use proconio::{input, marker::Usize1};
//
// fn main() {
//     input! {
//         n: usize,
//         m: usize,
//         edges: [(Usize1, Usize1); m],
//     };
//
//     let mut dsu = Dsu::new(n);
//     for (u, v) in edges {
//         dsu.merge(u, v);
//     }
//     println!("{}", dsu.groups().len());
// }

// alt: filter + count なら ans が不変になる・キューも BFS ごとに VecDeque::from で作ればスコープが閉じる
// let mut visited = vec![false; n];
// let ans = (0..n)
//     .filter(|&start| {
//         if visited[start] {
//             return false;
//         }
//         visited[start] = true;
//         let mut queue = VecDeque::from([start]);
//         while let Some(base) = queue.pop_front() {
//             for &x in &graph[base] {
//                 if !visited[x] {
//                     visited[x] = true;
//                     queue.push_back(x);
//                 }
//             }
//         }
//         true
//     })
//     .count();
