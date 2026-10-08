use cargo_snippet::snippet;

/// 1 次元累積和。`s[i] = a[0] + ... + a[i-1]`（`s[0] = 0`、長さ n + 1）を持ち、
/// 半開区間 `[l, r)` の和を O(1) で返す。
#[snippet("prefix_sum")]
pub struct PrefixSum(Vec<i64>);

#[snippet("prefix_sum")]
impl PrefixSum {
    pub fn new(a: &[i64]) -> Self {
        let mut s = vec![0];
        for &x in a {
            s.push(s.last().unwrap() + x);
        }
        Self(s)
    }

    /// `a[l] + ... + a[r-1]`。`l == r` なら 0
    pub fn sum(&self, l: usize, r: usize) -> i64 {
        self.0[r] - self.0[l]
    }
}

/// 2 次元累積和。`s[i][j]` = 行 `[0, i)` × 列 `[0, j)` の和（サイズ (h + 1) × (w + 1)）を持ち、
/// 長方形の和を O(1) で返す。
#[snippet("prefix_sum_2d")]
pub struct PrefixSum2d(Vec<Vec<i64>>);

#[snippet("prefix_sum_2d")]
impl PrefixSum2d {
    pub fn new(a: &[Vec<i64>]) -> Self {
        let h = a.len();
        let w = a.first().map_or(0, |row| row.len());
        let mut s = vec![vec![0; w + 1]; h + 1];
        // 横方向に累積してから縦方向に累積する (包除の 4 項を書かずに済む)
        for i in 0..h {
            for j in 0..w {
                s[i + 1][j + 1] = s[i + 1][j] + a[i][j];
            }
        }
        for i in 0..h {
            for j in 0..=w {
                s[i + 1][j] += s[i][j];
            }
        }
        Self(s)
    }

    /// 行 `[x1, x2)` × 列 `[y1, y2)` の和。幅か高さが 0 なら 0
    pub fn sum(&self, x1: usize, y1: usize, x2: usize, y2: usize) -> i64 {
        let s = &self.0;
        s[x2][y2] - s[x1][y2] - s[x2][y1] + s[x1][y1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 一次元_半開区間の和を返す() {
        let s = PrefixSum::new(&[1, 2, 2, 4, 5]);
        assert_eq!(s.sum(0, 3), 5);
        assert_eq!(s.sum(2, 5), 11);
        assert_eq!(s.sum(0, 5), 14);
        assert_eq!(s.sum(3, 4), 4);
    }

    #[test]
    fn 一次元_空区間は0() {
        let s = PrefixSum::new(&[1, 2, 3]);
        assert_eq!(s.sum(0, 0), 0);
        assert_eq!(s.sum(2, 2), 0);
        assert_eq!(s.sum(3, 3), 0);
    }

    #[test]
    fn 一次元_空配列でも作れる() {
        let s = PrefixSum::new(&[]);
        assert_eq!(s.sum(0, 0), 0);
    }

    #[test]
    fn 一次元_負の値を含んでもよい() {
        let s = PrefixSum::new(&[3, -5, 2, -1]);
        assert_eq!(s.sum(0, 4), -1);
        assert_eq!(s.sum(1, 3), -3);
    }

    // 1 2 3
    // 4 5 6
    // 7 8 9
    fn grid3() -> Vec<Vec<i64>> {
        vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]
    }

    #[test]
    fn 二次元_全体と部分長方形の和を返す() {
        let s = PrefixSum2d::new(&grid3());
        assert_eq!(s.sum(0, 0, 3, 3), 45);
        assert_eq!(s.sum(1, 1, 3, 3), 5 + 6 + 8 + 9);
        assert_eq!(s.sum(0, 1, 2, 2), 2 + 5);
        assert_eq!(s.sum(2, 0, 3, 3), 7 + 8 + 9);
    }

    #[test]
    fn 二次元_一マスはその値() {
        let s = PrefixSum2d::new(&grid3());
        for i in 0..3 {
            for j in 0..3 {
                assert_eq!(s.sum(i, j, i + 1, j + 1), grid3()[i][j]);
            }
        }
    }

    #[test]
    fn 二次元_幅か高さが0なら0() {
        let s = PrefixSum2d::new(&grid3());
        assert_eq!(s.sum(1, 1, 1, 3), 0);
        assert_eq!(s.sum(0, 2, 3, 2), 0);
    }

    #[test]
    fn 二次元_長方形のグリッドでも行と列を取り違えない() {
        // 2 行 3 列
        // 1 2 3
        // 4 5 6
        let s = PrefixSum2d::new(&[vec![1, 2, 3], vec![4, 5, 6]]);
        assert_eq!(s.sum(0, 0, 2, 3), 21);
        assert_eq!(s.sum(0, 1, 2, 3), 2 + 3 + 5 + 6);
        assert_eq!(s.sum(1, 0, 2, 2), 4 + 5);
    }

    #[test]
    fn 二次元_空グリッドでも作れる() {
        let s = PrefixSum2d::new(&[]);
        assert_eq!(s.sum(0, 0, 0, 0), 0);
    }
}
