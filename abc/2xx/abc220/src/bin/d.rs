// algo: dp, modint
// solve: copy 2026-10-20
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

const MOD: u64 = 998244353;

// 長さ N の列を先頭から F (和の一の位) か G (積の一の位) で 1 つずつ潰していき、最後に残る
// 値ごとに操作列の数を数える問題。操作列は 2^(N-1) 通りあり、N <= 10^5 では全列挙は論外。
//
// 操作の結果は先頭に戻るので、毎回の操作は「ここまでの計算結果」と次の A_i の計算になる。
// 以後の展開に効くのは今の先頭の値だけで、そこに至った操作の履歴は関係ない。しかも先頭の
// 値は % 10 で 0〜9 の 10 種類しかない。そこで履歴を捨てて
//   dp[v] = A_1..A_i まで潰した時点で先頭が v になっている操作列の数
// だけを持ち回す。同じ v にいる操作列は以後の振る舞いが全く同じなので、まとめて数えてよい。
// 1 本の操作列は次の一手で必ず F と G の 2 本に分かれるので、遷移は dp[v] を (v + x) % 10 と
// (v * x) % 10 の両方へ足すだけで、数え漏れも二重カウントも起きない。最後の dp[K] が
// 「最終値が K になる操作列の数」そのもの。
//
// 初期状態は「A_1 だけがあり操作は 0 回」なので dp[A_1] = 1 で、ループは A_2 から。
// 計算量は 1 ステップ 10 状態 × 2 遷移なので O(20N) ≈ 2×10^6。
//
// ndp を毎ステップ新しく作るのが要点。dp をその場で書き換えると、同じステップで足した値を
// また遷移元として使ってしまい、1 ステップで 2 回操作したことになる。
// 値は足すたびに % MOD するので常に MOD (≈ 10^9) 未満で、2 つ足しても 2×10^9 程度。u64 なら
// 余裕で収まる。
//
// 枝刈りの if dp[v] == 0 を dp[0] == 0 と書き間違えると、サンプルが片方だけ通る。sample1
// (A_1 = 2) は最初 dp[0] = 0 なので全遷移が continue されて全部 0 を出すが、sample2 (A_1 = 0)
// は dp[0] = 1 から始まり、以後も 0 * x = 0 で dp[0] が 0 にならないので全遷移が走って正解する。
// 状態が 10 個しかないのでこの枝刈りに速度上の意味はほぼなく、書かない方が安全 (下の alt)。

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut dp = [0u64; 10];
    dp[a[0]] = 1;

    for &x in &a[1..] {
        let mut ndp = [0u64; 10];
        for v in 0..10 {
            if dp[v] == 0 {
                continue;
            }
            let f = (v + x) % 10;
            let g = (v * x) % 10;
            ndp[f] = (ndp[f] + dp[v]) % MOD;
            ndp[g] = (ndp[g] + dp[v]) % MOD;
        }
        dp = ndp;
    }

    for v in dp {
        println!("{v}");
    }
}

// alt: 構造はそのまま (dp / ndp の持ち回しと v の 0..10 ループ)。削ったもの:
//      - if dp[v] == 0 { continue; } の枝刈り → 状態が 10 個なので速度は変わらず、
//        書き間違えると片方のサンプルだけ通る罠になる (上記)
//      - ndp[f] / ndp[g] の同じ形の 2 行 → 行き先を配列 [f, g] にして for で 1 行に
//      - 一度しか使わない変数 f, g → 配列リテラルに直接書く
// let mut dp = [0u64; 10];
// dp[a[0]] = 1;
// for &x in &a[1..] {
//     let mut ndp = [0u64; 10];
//     for v in 0..10 {
//         for to in [(v + x) % 10, (v * x) % 10] {
//             ndp[to] = (ndp[to] + dp[v]) % MOD;
//         }
//     }
//     dp = ndp;
// }

// alt: fold で dp を持ち回すと let mut dp と dp = ndp の入れ替えが消え、「1 ステップ =
//      長さ 10 の配列 → 配列の変換」という形がそのままコードに出る。enumerate で
//      (v, 通り数) を受け取れば dp[v] の添字アクセスも消える
// let mut init = [0u64; 10];
// init[a[0]] = 1;
// let dp = a[1..].iter().fold(init, |dp, &x| {
//     let mut ndp = [0u64; 10];
//     for (v, &c) in dp.iter().enumerate() {
//         ndp[(v + x) % 10] = (ndp[(v + x) % 10] + c) % MOD;
//         ndp[(v * x) % 10] = (ndp[(v * x) % 10] + c) % MOD;
//     }
//     ndp
// });
