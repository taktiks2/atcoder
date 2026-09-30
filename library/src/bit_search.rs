use cargo_snippet::snippet;

/// bit 全探索。`0..m` の部分集合を、選んだ添字の昇順リストとして全 2^m 通り返す。
/// 並びは mask = 0, 1, ..., 2^m - 1 の順（先頭は空集合）。
/// mask の i ビット目が 1 なら i を選ぶ。m <= 20 程度が目安。
#[snippet("bit_search")]
pub fn subsets(m: usize) -> Vec<Vec<usize>> {
    (0..1usize << m)
        .map(|mask| (0..m).filter(|&i| mask >> i & 1 == 1).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn m0なら空集合のみ() {
        assert_eq!(subsets(0), vec![Vec::<usize>::new()]);
    }

    #[test]
    fn m3ならmask順に8通り() {
        let expected: Vec<Vec<usize>> = vec![
            vec![],
            vec![0],
            vec![1],
            vec![0, 1],
            vec![2],
            vec![0, 2],
            vec![1, 2],
            vec![0, 1, 2],
        ];
        assert_eq!(subsets(3), expected);
    }

    #[test]
    fn 個数は2のm乗() {
        assert_eq!(subsets(10).len(), 1 << 10);
    }
}
