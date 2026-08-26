use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

// 多始点 BFS

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        s: [Chars; h],
    };

    // 1. 各行・各列に爆弾があるかを前計算
    let mut row_has_bomb = vec![false; h];
    let mut col_has_bomb = vec![false; w];
    for (i, row) in s.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == '#' {
                row_has_bomb[i] = true;
                col_has_bomb[j] = true;
            }
        }
    }

    // 2. 安全マスをすべて距離 0 でキューに入れる (多始点 BFS の始点)
    const INF: usize = usize::MAX; // 番兵
    let mut dist = vec![vec![INF; w]; h];
    let mut queue = VecDeque::new();
    for (i, row) in s.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == '.' && !row_has_bomb[i] && !col_has_bomb[j] {
                dist[i][j] = 0;
                queue.push_back((i, j));
            }
        }
    }

    // 3. BFS で「最寄りの安全マスまでの距離」を確定させる
    let dy: [isize; 4] = [0, 0, -1, 1];
    let dx: [isize; 4] = [-1, 1, 0, 0];
    while let Some((i, j)) = queue.pop_front() {
        let d = dist[i][j];
        if d == k {
            continue;
        }
        for dir in 0..4 {
            let ni = i as isize + dy[dir];
            let nj = j as isize + dx[dir];
            if ni < 0 || ni >= h as isize || nj < 0 || nj >= w as isize {
                continue;
            }
            let (ni, nj) = (ni as usize, nj as usize);
            if s[ni][nj] == '.' && dist[ni][nj] == INF {
                dist[ni][nj] = d + 1;
                queue.push_back((ni, nj));
            }
        }
    }

    // 4. 距離がK以下のマスを数える
    let ans = dist
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&d| d <= k) // "<=" とすることでINFを弾ける
        .count();
    println!("{}", ans);
}
