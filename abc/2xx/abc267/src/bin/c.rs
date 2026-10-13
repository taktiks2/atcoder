// algo: sliding-window, prefix-sum
// solve: hint 2026-10-13
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 長さ N の数列から連続する長さ M の部分列を選び、各項に 1, 2, ..., M の重みを
// 掛けて足し合わせたスコアを最大化する問題。
//
// 長さ M の区間は (N - M + 1) 個ある。各区間について愚直に重み付き和を計算すると
// 1 回あたり O(M) かかり、全体で O((N - M + 1) * M) ≈ 4×10^10。2 秒制限（約 2×10^8 回）
// に対して大幅に間に合わず TLE になる。
//
// 区間を 1 つ右へスライドさせたときのスコアの変化量（差分）に着目する。
// 直前の区間を A[k..k+M]、次の区間を A[k+1..k+1+M] とすると、
//   次のスコア - 直前のスコア
//   = (1*A[k+1] + 2*A[k+2] + ... + M*A[k+M]) - (1*A[k] + 2*A[k+1] + ... + M*A[k+M-1])
//   = M * A[k+M] - (A[k] + A[k+1] + ... + A[k+M-1])
// つまり「新しく入る要素を M 倍して足し、直前の区間の総和を丸ごと引く」だけで更新できる。
// 各要素の係数が 1 ずつ下がるため直前の区間和が引かれる形になり、掛ける数値が変わる
// 計算のやり直しを回避して O(1) で追従できる。
//
// 型は i64 が必須。A_i の最大値は 2×10^5、M は最大 2×10^5 で、スコアの最大値は
// およそ (M^2 / 2) * max(A) ≈ 4×10^15 に達する。32-bit 符号付き整数の上限
// (約 2.1×10^9) を大きく超えるため、累積和・スコア・答えのすべてを i64 で扱う。
//
// 落とし穴は A_i が負になり得ること（A_i >= -2×10^5）。スコアの最大値が負になるケースが
// あるため、初期値を 0 にすると全区間が負の場合に 0 を出力して WA になる。
// 初期値を最初の区間のスコア `ans` に置いているため安全。

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; n],
    };

    let mut ans: i64 = a[..m]
        .iter()
        .enumerate()
        .map(|(i, &x)| (i as i64 + 1) * x)
        .sum();

    // テストケースとRustのLLVMの自動ベクトル化の恩恵によってたまたまACできた
    let prefix_sum: Vec<i64> = a.windows(m).map(|x| x.iter().sum::<i64>()).collect();

    let mut first = ans;
    for i in m..n {
        first = first - prefix_sum[i - m] + a[i] * m as i64;
        ans = ans.max(first);
    }

    println!("{ans}");
}

// alt: 構造はそのまま、不要な中間ベクタを削った版
//      - `prefix_sum` の中間 Vec 確保 (メモリ O(N)) を削り、区間和も同時に O(1) でスライド更新
//      - `windows(m).map(|x| x.iter().sum())` による最悪 O(NM) の走査を解消し、真の O(N) に
// let mut sum: i64 = a[..m].iter().sum();
// let mut cur: i64 = a[..m]
//     .iter()
//     .enumerate()
//     .map(|(i, &x)| (i as i64 + 1) * x)
//     .sum();
// let mut ans = cur;
// for i in m..n {
//     cur += a[i] * m as i64 - sum;
//     sum += a[i] - a[i - m];
//     ans = ans.max(cur);
// }
// println!("{ans}");

// alt: あらかじめ全体の累積和を求めておく版。各スライドで引くべき区間和を
//      pref[i] - pref[i - m] で直接取得できるため、「直前の区間和を引く」という
//      数式通りの構造が明確になり、スライド更新の順序や添字の混乱を防げる
// let mut pref = vec![0i64; n + 1];
// for i in 0..n {
//     pref[i + 1] = pref[i] + a[i];
// }
// let mut cur: i64 = a[..m]
//     .iter()
//     .enumerate()
//     .map(|(i, &x)| (i as i64 + 1) * x)
//     .sum();
// let mut ans = cur;
// for i in m..n {
//     cur += a[i] * m as i64 - (pref[i] - pref[i - m]);
//     ans = ans.max(cur);
// }
// println!("{ans}");

// alt: 2 重累積和（累積和の累積和）により、スライドのループを回さず各区間を O(1) で
//      一発計算する版。S を A の累積和、T を S の累積和とすると、区間 [l, l+M) の
//      スコアは M * S[l+M] - (T[l+M] - T[l]) と表せる。差分更新の初期値計算すら不要
// let mut s = vec![0i64; n + 1];
// for i in 0..n {
//     s[i + 1] = s[i] + a[i];
// }
// let mut t = vec![0i64; n + 1];
// for i in 0..n {
//     t[i + 1] = t[i] + s[i];
// }
// let mut ans = i64::MIN;
// for l in 0..=n - m {
//     let val = m as i64 * s[l + m] - (t[l + m] - t[l]);
//     ans = ans.max(val);
// }
// println!("{ans}");
