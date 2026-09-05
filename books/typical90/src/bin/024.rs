// algo: math, parity
use proconio::input;

// A から B にちょうど K 回で到達できるかを判定する。
//
// 最小必要回数は各要素の差の絶対値の和 total = Σ |a_i - b_i|。余った (K - total) 回は
// 「同じ要素を +1 してすぐ -1」のように 2 回一組で打ち消せる (無駄遣い) が、1 回だけ
// 余ると消せない。よって K >= total かつ (K - total) が偶数、が必要十分。
// 言い換えると total と K のパリティが一致していれば足りる。

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
        b: [i64; n],
    };

    let total: i64 = (0..n).map(|i| (a[i] - b[i]).abs()).sum();

    dbg!(total, k);

    println!(
        "{}",
        if total <= k && (k - total) % 2 == 0 {
            "Yes"
        } else {
            "No"
        }
    );
}

// alt: 添え字を回すより zip でペアにする方が意図が読みやすい。i64::abs は i64::MIN で
//      panic するが、この問題は A_i, B_i が制約に収まるので安全
// let total: i64 = a.iter().zip(&b).map(|(x, y)| (x - y).abs()).sum();

// alt: (K - total) % 2 == 0 は「total と K のパリティ一致」と同値。引き算せずに済み、
//      total > K のときも副作用なく評価できるので、条件を 1 本の式にできる
// if total <= k && total % 2 == k % 2 { "Yes" } else { "No" }
