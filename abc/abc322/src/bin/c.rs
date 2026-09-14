// algo: binary-search
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 「i 日目以降で最初に花火が上がるのは何日後か」は、A の中から「i 以上で最小の要素」を
// 見つけて A_j - i を答える問題に言い換えられる。A_M = N が保証されているので「i 以降に
// 花火が無い」ケースは存在せず、見つからなかった場合の場合分けは要らない。
//
// 日ごとに A を先頭から舐めると O(NM)。N = M = 2×10^5 だと 4×10^10 で当然 TLE。
// A が昇順に与えられている (A_1 < A_2 < ... < A_M) のが本質で、二分探索すれば
// 1 日あたり O(log M) ≈ 18 回の比較、全体で 2×10^5 × 18 ≈ 3.6×10^6 に落ちる。
//
// めぐる式の ok / ng は「条件 a[idx] >= day を満たす添字 / 満たさない添字」と定義した。
// a は昇順なので判定結果は false...false true...true と単調に並び、二分探索が成立する。
// 端の初期値は「必ず ok」「必ず ng」と言い切れる位置に置く必要がある:
//   ok = m - 1 ... a[m-1] = N >= day は常に真 (day <= N)
//   ng = -1    ... day = 1 のときは全要素が条件を満たすので、配列内に ng は存在しない。
//                  範囲外を番兵にすることで「存在しない場合」の分岐を消している
// 番兵に -1 を使う都合で ok / ng は isize。usize のままだと ok - ng がアンダーフローする
// (その場合は abs_diff を使う)。
//
// ループ条件 (ok - ng).abs() > 1 は「隣り合うまで詰める」の意。abs なのは、条件の向きが
// 逆の問題 (「x 以下で最大」など) では ok が左に来るためで、同じ型で書き回すための保険。
// mid は必ずループの内側で計算し直す。外に出すと区間が縮まず無限ループになる。
//
// 出力は N 行 = 最大 2×10^5 行。println! は行ごとに write システムコールを呼ぶので
// #[fastout] (内部で BufWriter 相当) を必ず付ける。

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    };

    for day in 1..=n {
        // 条件: a[idx] >= day
        let mut ok: isize = (m - 1) as isize; // a[m-1] = N なので必ず満たす
        let mut ng: isize = -1; // 範囲外の番兵。必ず満たさない扱い

        while (ok - ng).abs() > 1 {
            let mid = (ok + ng) / 2;
            if a[mid as usize] >= day {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        println!("{}", a[ok as usize] - day);
    }
}

// alt: 自前の ok / ng は partition_point (C++ の lower_bound 相当) にそのまま置き換えられる。
//      述語には「まだ day 未満」を渡すので、返るのは条件を満たす要素数 = day 以上の最初の添字。
//      番兵も isize も要らなくなり、off-by-one を std に任せられるので実戦ではこちらが速い
// for day in 1..=n {
//     let idx = a.partition_point(|&firework| firework < day);
//     println!("{}", a[idx] - day);
// }

// alt: そもそも二分探索が要らない。最終日から逆順に降りていき「直近の花火の日」を持ち回せば
//      O(N + M)。A を bool 配列 is_firework に展開しておき、花火の日に来たら next を更新する。
//      day が 1 ずつ減るので next - day がそのまま答え。log が落ちる上に実装も短い
// let mut is_firework = vec![false; n + 1]; // 日付をそのまま添字に使う (0 は未使用)
// for d in a {
//     is_firework[d] = true;
// }
// let mut ans = vec![0; n + 1];
// let mut next = n; // A_M = N なので最終日は必ず花火
// for day in (1..=n).rev() {
//     if is_firework[day] {
//         next = day;
//     }
//     ans[day] = next - day;
// }
// for day in 1..=n {
//     println!("{}", ans[day]);
// }

// alt: day が 1 から単調に増えるので、探索位置 j を日ごとにリセットせず持ち越せば尺取りになる。
//      j は全体で高々 M 回しか進まないので O(N + M)。二分探索で「毎日 ok / ng を初期化して
//      いる」のが実は無駄だった、という見方ができる。A_M = N のおかげで j が範囲外に出ない
// let mut j = 0;
// for day in 1..=n {
//     while a[j] < day {
//         j += 1;
//     }
//     println!("{}", a[j] - day);
// }
