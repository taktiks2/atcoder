use cargo_snippet::snippet;

/// めぐる式二分探索。`ok` は条件を満たす側、`ng` は満たさない側の初期値。
/// `is_ok` が単調（境界を挟んで true/false が切り替わる）なら、
/// 条件を満たす限界の値を返す。`ok < ng` / `ok > ng` どちらの向きでも使える。
#[snippet("binary_search")]
pub fn meguru_bisect(mut ok: i64, mut ng: i64, is_ok: impl Fn(i64) -> bool) -> i64 {
    while (ok - ng).abs() > 1 {
        let mid = (ok + ng) / 2;
        if is_ok(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ok
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 上限を探す_okが左でngが右() {
        // x^2 <= 25 を満たす最大の x
        assert_eq!(meguru_bisect(0, 100, |x| x * x <= 25), 5);
    }

    #[test]
    fn 下限を探す_okが右でngが左() {
        // ソート済み配列で a[i] >= 4 となる最小の添字 (lower_bound)
        let a = [1, 3, 5, 7, 9];
        let idx = meguru_bisect(a.len() as i64, -1, |i| a[i as usize] >= 4);
        assert_eq!(idx, 2);
    }

    #[test]
    fn 述語が全域でtrueなら境界までokが進む() {
        // すべての要素が 100 以下 → 満たす最大の添字は末尾
        let a = [10, 20, 30];
        assert_eq!(meguru_bisect(0, a.len() as i64, |i| a[i as usize] <= 100), 2);
    }
}
