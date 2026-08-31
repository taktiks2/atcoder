use cargo_snippet::snippet;

/// 幅優先探索。隣接リスト `g` 上で `start` からの最短距離（辺数）を返す。
/// 未到達の頂点は -1。
#[snippet]
pub fn bfs(g: &[Vec<usize>], start: usize) -> Vec<i64> {
    use std::collections::VecDeque;
    let mut dist = vec![-1i64; g.len()];
    let mut queue = VecDeque::new();
    dist[start] = 0;
    queue.push_back(start);
    while let Some(v) = queue.pop_front() {
        for &to in &g[v] {
            if dist[to] == -1 {
                dist[to] = dist[v] + 1;
                queue.push_back(to);
            }
        }
    }
    dist
}

#[cfg(test)]
mod tests {
    use super::*;

    // 0-1, 0-2, 1-3 の無向グラフ + 孤立点 4
    fn sample_graph() -> Vec<Vec<usize>> {
        vec![vec![1, 2], vec![0, 3], vec![0], vec![1], vec![]]
    }

    #[test]
    fn 始点からの距離を返し未到達はマイナス1() {
        assert_eq!(bfs(&sample_graph(), 0), vec![0, 1, 1, 2, -1]);
    }

    #[test]
    fn 始点を変えても正しい() {
        assert_eq!(bfs(&sample_graph(), 3), vec![2, 1, 3, 0, -1]);
    }

    #[test]
    fn 孤立点が始点なら自分だけ0() {
        assert_eq!(bfs(&sample_graph(), 4), vec![-1, -1, -1, -1, 0]);
    }
}
