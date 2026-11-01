// algo: sort, binary-search
// solve: hint 2026-11-01
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// H×W のグリッドに置かれた N 個の駒について、「u〜d 行目にある駒の数」または
// 「l〜r 列目にある駒の数」を Q 回答える問題。
//
// 行クエリでは駒の列番号をまったく使わず、列クエリでは行番号を使わない。なので 2 次元の
// 問題に見えて、実際は「行番号の列」と「列番号の列」についての 1 次元の問題が 2 つあるだけ。
// H, W <= 10^9 なので、長さ H の配列に駒の数を数えて累積和、という方法は使えない
// (10^9 要素は作れない)。また、1 クエリごとに N 個を全部見ると 3*10^5 * 3*10^5 = 9*10^10 で間に合わない。
//
// 値の列を昇順にソートしておけば、「x 未満の要素」はソート列の先頭にまとまっている。
// partition_point(|&v| v < u) は述語が true の要素の数、つまり u 未満の要素数を
// O(log N) で返す。同じように partition_point(|&v| v <= d) は d 以下の要素数を返す。
// 区間 [u, d] に入る要素数 = (d 以下の要素数) - (u 未満の要素数)。
// u <= d なので後者が前者を超えることはなく、usize の引き算でもアンダーフローしない。
// 計算量はソートの O(N log N) と各クエリの O(log N) で、全体 O((N + Q) log N)。
//
// 落とし穴: 2 回の partition_point は必ず同じ配列に対して行う。片方を sorted_r、
// もう片方を sorted_c にすると (書きかけで実際にやった)、差に意味が無くなり、
// 下端側の方が大きくなったときに usize がアンダーフローして panic する。
// また、下端の述語は `<`、上端の述語は `<=` にする。両方を `<` にすると
// 値がちょうど d の駒を数え落とす。

#[fastout]
fn main() {
    input! {
        _: usize,
        _: usize,
        n: usize,
        q: usize,
        rc: [(usize, usize); n],
    };

    let mut sorted_r = vec![];
    let mut sorted_c = vec![];
    for (ri, ci) in rc {
        sorted_r.push(ri);
        sorted_c.push(ci);
    }
    sorted_r.sort_unstable();
    sorted_c.sort_unstable();

    dbg!(&sorted_r, &sorted_c);

    for _ in 0..q {
        input! { t: usize };

        match t {
            1 => {
                input! { u: usize, d: usize };

                let ui = sorted_r.partition_point(|&x| x < u);
                let di = sorted_r.partition_point(|&x| x <= d);

                println!("{}", di - ui);
            }
            2 => {
                input! { l: usize, r: usize };

                let li = sorted_c.partition_point(|&x| x < l);
                let ri = sorted_c.partition_point(|&x| x <= r);

                println!("{}", ri - li);
            }
            _ => unimplemented!(),
        }
    }
}

// alt: 構造はそのまま (ソート 2 本 + partition_point の差) で冗長な部分を削った版
//      - push ループで 2 本の Vec に分けていた所 → unzip で 1 行
//      - match の 2 本の腕が配列以外は同じ → 配列を参照で選び、本体を 1 つにまとめる
//      - dbg! と unimplemented! の腕を削除 (t は 1 か 2 しか来ない)
// let (mut rs, mut cs): (Vec<usize>, Vec<usize>) = rc.into_iter().unzip();
// rs.sort_unstable();
// cs.sort_unstable();
//
// for _ in 0..q {
//     input! { t: usize, lo: usize, hi: usize };
//     let v = if t == 1 { &rs } else { &cs };
//     let ans = v.partition_point(|&x| x <= hi) - v.partition_point(|&x| x < lo);
//     println!("{ans}");
// }

// alt: 「x 以下の個数」の関数を 1 つだけ用意し、区間を「累積和の差」の形で書く版。
//      述語が `<=` の 1 種類だけになり、`<` と `<=` の使い分けを間違えようがない。
//      lo >= 1 なので lo - 1 も usize のまま安全
// let count_le = |v: &[usize], x: usize| v.partition_point(|&y| y <= x);
// for _ in 0..q {
//     input! { t: usize, lo: usize, hi: usize };
//     let v = if t == 1 { &rs } else { &cs };
//     println!("{}", count_le(v, hi) - count_le(v, lo - 1));
// }
