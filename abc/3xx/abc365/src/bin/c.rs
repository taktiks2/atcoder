// algo: binary-search
// solve: copy 2026-11-15
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 上限額 x を決めると人 i には min(x, A_i) 円渡る。総額が M 以下に収まる最大の x を求める問題。
//
// f(x) = Σ min(x, A_i) は x について単調非減少 (x を上げて支給額が減る人はいない)。
// だから「f(x) <= M か」は、ある境目まで true、その先はずっと false に分かれる。
// この境目を探すのが本題。
//
// x を 1 ずつ上げて試すのは落ちる。判定 1 回が O(N) で、答えは最大 2×10^14 近くまで
// 行きうる (N = 1, A_1 = 10^9 > M のような形でも答えは M)。A が「1 が 199999 人と
// 10^9 が 1 人」、M = 5×10^8 なら、M / N = 2500 から数えても約 5×10^8 回 × 2×10^5 になる。
// 境目を二分探索すれば判定は log2(2×10^14) ≒ 48 回、全体で約 10^7。
//
// infinite の条件は「全員が M / N 以下」ではなく ΣA <= M。x >= max(A) になると
// f(x) = ΣA で頭打ちになり、それが予算内なら x をいくら上げても超えない。
// 「全員が M / N 以下」だと N = 2, M = 10, A = [1, 9] (ΣA = 10 <= M で infinite) を
// 取りこぼす。サンプル 2 はどちらの条件でも通るので、サンプルでは気づけない。
//
// 番兵: ok = 0 は f(0) = 0 <= M で必ず true。ng = M + 1 は、infinite を除いた後
// (ΣA > M) なら必ず false。A_i >= M + 1 の人が 1 人でもいればその人だけで M + 1、
// いなければ f(M + 1) = ΣA > M だから。ng = M にすると N = 1, M = 5, A = [9] の
// 答え 5 を取りこぼす。
//
// 型: ΣA は最大 2×10^5 × 10^9 = 2×10^14 で、i32 / u32 (約 2.1×10^9 / 4.3×10^9) は溢れる。
// 候補の x を Vec に並べて partition_point するのも不可 (最大 2×10^14 要素)。
// 配列を作らず ok / ng の 2 変数だけで狭める。

#[fastout]
fn main() {
    input! {
        n: usize,
        m: u64,
        a: [u64; n],
    };

    if a.iter().sum::<u64>() <= m {
        println!("infinite");
        return;
    }

    let mut ok = 0u64;
    let mut ng = m + 1;
    while ng - ok > 1 {
        let mid = (ok + ng) / 2;
        if a.iter().map(|&x| x.min(mid)).sum::<u64>() <= m {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{ok}");
}

// alt: リファクタリング版。構造はそのまま (infinite 判定 → ok / ng の二分探索)。
//      - 2 か所に書いていた総額の計算をクロージャ total にまとめた
//      - ΣA は「上限なしの総額」= total(u64::MAX) として同じ関数で表せる
//      「判定式に名前を付ける」だけで、二分探索の本体が total(mid) <= m と読めるようになる
// let total = |x: u64| a.iter().map(|&v| v.min(x)).sum::<u64>();
// if total(u64::MAX) <= m {
//     println!("infinite");
//     return;
// }
// let mut ok = 0u64;
// let mut ng = m + 1;
// while ng - ok > 1 {
//     let mid = (ok + ng) / 2;
//     if total(mid) <= m {
//         ok = mid;
//     } else {
//         ng = mid;
//     }
// }
// println!("{ok}");

// alt: ソート + 累積で境目を直接求める。計算量が M の大きさに依存しない O(N log N) になり、
//      ok / ng の番兵を考えなくてよい。A を昇順に並べ「小さい方から i 人は満額、残り
//      n - i 人は一律 x 円」と考える。x を a[i] まで上げたときの総額 pre + a[i] * (n - i) が
//      初めて M を超える i で、残り予算 M - pre を n - i 人で割った値が答え。
//      最後まで超えなければ ΣA <= M なので infinite (判定を別に書かなくて済む)
// let mut a = a;
// a.sort_unstable();
// let mut pre = 0u64; // a[0..i] の合計
// for i in 0..n {
//     let rest = (n - i) as u64;
//     if pre + a[i] * rest > m {
//         println!("{}", (m - pre) / rest);
//         return;
//     }
//     pre += a[i];
// }
// println!("infinite");
