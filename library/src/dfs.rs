use cargo_snippet::snippet;

/// 深さ優先探索（再帰）。隣接リスト `g` 上で `start` から到達できる頂点を
/// 行きがけ順（訪問順）で返す。
#[snippet]
pub fn dfs(g: &[Vec<usize>], start: usize) -> Vec<usize> {
    fn go(v: usize, g: &[Vec<usize>], visited: &mut [bool], order: &mut Vec<usize>) {
        visited[v] = true;
        order.push(v);
        for &to in &g[v] {
            if !visited[to] {
                go(to, g, visited, order);
            }
        }
    }
    let mut visited = vec![false; g.len()];
    let mut order = Vec::new();
    go(start, g, &mut visited, &mut order);
    order
}

#[cfg(test)]
mod tests {
    use super::*;

    // 0-1, 0-2, 1-3 の無向グラフ + 孤立点 4
    fn sample_graph() -> Vec<Vec<usize>> {
        vec![vec![1, 2], vec![0, 3], vec![0], vec![1], vec![]]
    }

    #[test]
    fn 行きがけ順で訪問する() {
        assert_eq!(dfs(&sample_graph(), 0), vec![0, 1, 3, 2]);
    }

    #[test]
    fn 到達できない頂点は含まれない() {
        let order = dfs(&sample_graph(), 0);
        assert!(!order.contains(&4));
    }

    #[test]
    fn 孤立点が始点なら自分だけ() {
        assert_eq!(dfs(&sample_graph(), 4), vec![4]);
    }
}
