// algo: prefix-sum
// solve: hint 2026-10-12
#![allow(unused_imports)]
use indexing::algorithms::binary_search;
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// マス s から t へ移動する最小コスト。マス i からは左のどのマスにも L_i、右のどのマスにも
// R_i で飛べる。N, Q <= 3×10^5 なので、クエリごとにグラフ探索をすると O(NQ) ≈ 9×10^10
// で間に合わない。クエリあたり O(1) で答える形まで落とす必要がある。
//
// t は「s の右か左か」という向きにしか効かない。s < t の場合、最初の一手は次のどちらか:
//   - 右へ飛ぶ (R_s): どこにでも着地できるので t に直接着いて終わり
//   - 左へ飛ぶ (L_s): 着地点 k < s を自由に選べ、そこから R_k で t に直接飛べる
// 3 手以上は得にならない (左へ 2 回飛んでも行き先の候補は増えず、コストが足されるだけ)。
// よって答えは min(R_s, L_s + min_{k<s} R_k) で、min_{k<s} R_k は R の累積 min を前計算して
// O(1) で引ける。s > t は左右を入れ替えて min(L_s, R_s + min_{k>s} L_k) を L の
// 後ろからの累積 min で引く。
//
// 累積 min は s 自身を含んでいる (r_mins[s] = min(R_0..=R_s)) が問題ない。k = s を選ぶと
// L_s + R_s になり、直行の R_s に必ず負けるので答えを壊さない。番兵として ll[0] = 0 と
// r[n-1] = 0 を足しているのも同じ理由で安全。マス 1 から右へ行くとき r_mins[0] + ll[0]
// = R_0 で直行と同値、マス N から左へ行くとき l_mins[n-1] + r[n-1] = L_{n-1} で同値になる。
// どちらの番兵も「範囲外の向き」でしか参照されない (s < t なら s <= n-2 なので r の番兵は
// 累積 min に入らない)。
//
// 落とし穴は「左へ飛んだ先 k は s より左」という制約を忘れて、R 全体の min を使うこと。
// 右にある安い R_k は、一度左へ飛んでしまうと使えない。この誤りでもサンプルは全部通る
// (手元のランダムテストで 500 ケース中 53 ケースが WA)。
//
// 値は最大で L + R = 2×10^9。i32 の上限 2.1×10^9 にぎりぎり収まるが余裕は無いので、
// usize のままで良い。
// 出力は Q 行なので #[fastout] がまとめて書き出すのが効いている。
// dbg! は標準エラー出力に出るのでジャッジは無視するが、3×10^5 要素を書き出す分だけ遅い。

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        l: [usize; n - 1],
        mut r: [usize; n - 1],
        query: [(Usize1, Usize1); q],
    };

    let mut ll: Vec<usize> = vec![0];
    ll.extend(l);
    r.push(0);

    let mut l_mins: Vec<usize> = vec![0; n];
    let mut lmin = usize::MAX;
    for (i, &x) in ll.iter().enumerate().rev() {
        lmin = lmin.min(x);
        l_mins[i] = lmin;
    }

    let mut r_mins: Vec<usize> = vec![0; n];
    let mut rmin = usize::MAX;
    for (i, &x) in r.iter().enumerate() {
        rmin = rmin.min(x);
        r_mins[i] = rmin;
    }

    dbg!(&l_mins, &r_mins);

    for (s, t) in query {
        if s < t {
            // 右方向
            if r[s] <= ll[s] {
                // 直行
                println!("{}", r[s]);
            } else {
                if r_mins[s] + ll[s] < r[s] {
                    // 経由
                    println!("{}", r_mins[s] + ll[s]);
                    continue;
                }
                // 直行
                println!("{}", r[s]);
            }
        } else {
            // 左方向
            if ll[s] <= r[s] {
                // 直行
                println!("{}", ll[s]);
            } else {
                if l_mins[s] + r[s] < ll[s] {
                    // 経由
                    println!("{}", l_mins[s] + r[s]);
                    continue;
                }
                // 直行
                println!("{}", ll[s]);
            }
        }
    }
}

// alt: 構造はそのまま (ll / r の番兵・累積 min の 2 ループ・向きの分岐)。削ったのは:
//      - 「直行が安い / 経由が安い」の if 4 本 → min 1 回。どちらも候補の小さい方を出すだけ
//      - continue と、同じ println! が 2 回ずつあった重複
//      - dbg!
// (main 内の input! から for までを、分岐部分だけ次の形に)
// for (s, t) in query {
//     if s < t {
//         println!("{}", r[s].min(r_mins[s] + ll[s]));
//     } else {
//         println!("{}", ll[s].min(l_mins[s] + r[s]));
//     }
// }

// alt: 「s 自身を含まない」累積 min を scan で作る版。式 min_{k<s} R_k が添字の意味と
//      そのまま一致し、番兵にも「使えないマス = 大きい値」という意味がつくので、0 を詰めて
//      安全性を確かめる議論が要らなくなる。big = usize::MAX / 2 にしているのは、big 同士を
//      足してもオーバーフローしないため
// let big = usize::MAX / 2;
// let lc: Vec<usize> = std::iter::once(big).chain(l).collect();
// let rc: Vec<usize> = r.into_iter().chain(std::iter::once(big)).collect();
// // pre_r[i] = min(R[0..i])、suf_l[i] = min(L[i+1..])
// let pre_r: Vec<usize> = rc
//     .iter()
//     .scan(big, |m, &x| {
//         let cur = *m;
//         *m = (*m).min(x);
//         Some(cur)
//     })
//     .collect();
// let mut suf_l: Vec<usize> = lc
//     .iter()
//     .rev()
//     .scan(big, |m, &x| {
//         let cur = *m;
//         *m = (*m).min(x);
//         Some(cur)
//     })
//     .collect();
// suf_l.reverse();
// for (s, t) in query {
//     let ans = if s < t {
//         rc[s].min(lc[s] + pre_r[s])
//     } else {
//         lc[s].min(rc[s] + suf_l[s])
//     };
//     println!("{ans}");
// }
