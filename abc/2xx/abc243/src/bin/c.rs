// algo: sort, set
// solve: self 2026-10-16
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// x 軸方向に歩く N 人の中で、逆向きの二人が出会うかを判定する問題。
// 全ペアを調べると N(N-1)/2 ≈ 2*10^10 (N = 2*10^5) 通りで間に合わない。
//
// 衝突するのは y が同じ二人だけで、条件は「左側の人が R、右側の人が L」。
// 間に別の人がいても関係ない。全員が同じ速さで動くので、同じ向きの人同士の
// 間隔は変わらず、R と L の間は必ず詰まる。つまり y ごとに見て、
// 「x が小さい順に並べたとき、R の後に L が一度でも現れるか」だけが問題になる。
//
// そこで (x, y, 向き) を x の昇順にソートし、左から見ていく。set には
// 「すでに R が出た y」を入れる。L が来たときにその y が set にあれば、左に R が
// いるので衝突する。全員が異なる地点にいるので、x と y が両方等しい要素は無く、
// 同じ x の要素がどんな順に並んでも判定は変わらない (y が違えば互いに無関係)。
//
// 落とし穴は向きの組み合わせ。L が左で R が右の場合は離れていくだけなので、
// 「同じ y に R と L が両方いれば Yes」とすると WA になる (sample4: (1,3) が L、
// (3,3) が R で No)。x, y <= 10^9 は足し引きせず比較するだけなので usize で足りる。
//
// 計算量はソートの O(N log N) が支配的で、走査は HashSet で平均 O(N)。

#[fastout]
fn main() {
    input! {
        n: usize,
        xy: [(usize, usize); n],
        s: Chars,
    };

    let mut xys: Vec<(usize, usize, char)> = vec![(0, 0, 'a'); n];
    for (i, (xi, yi)) in xy.iter().enumerate() {
        xys[i] = (*xi, *yi, s[i]);
    }

    xys.sort_unstable();

    let mut set: HashSet<usize> = HashSet::new();

    let mut ans = "No";
    for (_, yi, ci) in xys {
        if set.contains(&yi) {
            // 最初に右が出てから一つでも左が出れば衝突
            if ci == 'L' {
                ans = "Yes";
                break;
            }
        } else {
            // Xが少ない方から見ていって最初の右向き
            if ci == 'R' {
                set.insert(yi);
            }
        }
    }

    println!("{ans}");
}

// alt: 構造はそのまま (ソートして左から走査、R が出た y を set に入れる)。冗長な所だけ削る
//      - 0 で初期化した Vec + 添字ループ → zip + map + collect で直接組み立てる
//      - contains / 向きの if を入れ子にした 4 分岐 → 向きで分ける if / else if の 2 分岐
//        (R ならいつでも insert してよい。すでに入っていれば何も起きないだけ)
// let mut xys: Vec<(usize, usize, char)> =
//     xy.iter().zip(&s).map(|(&(x, y), &c)| (x, y, c)).collect();
// xys.sort_unstable();
//
// let mut set: HashSet<usize> = HashSet::new();
//
// let mut ans = "No";
// for (_, yi, ci) in xys {
//     if ci == 'R' {
//         set.insert(yi);
//     } else if set.contains(&yi) {
//         ans = "Yes";
//         break;
//     }
// }
//
// println!("{ans}");

// alt: ソート不要で平均 O(N)。y ごとに「R の最小 x」と「L の最大 x」だけ持てば、
//      最小の R < 最大の L のとき、かつそのときに限り衝突する。いちばん左の R と
//      いちばん右の L の組み合わせで判定できるので、並べ替える必要がない。
//      entry + `*e = (*e).min(x)` は「&mut の中身を書き換えるときは * が要る」の典型例
// let mut min_r: HashMap<usize, usize> = HashMap::new();
// let mut max_l: HashMap<usize, usize> = HashMap::new();
// for (&(x, y), &c) in xy.iter().zip(&s) {
//     if c == 'R' {
//         let e = min_r.entry(y).or_insert(x);
//         *e = (*e).min(x);
//     } else {
//         let e = max_l.entry(y).or_insert(x);
//         *e = (*e).max(x);
//     }
// }
// let ans = min_r.iter().any(|(y, &r)| max_l.get(y).is_some_and(|&l| r < l));
// println!("{}", if ans { "Yes" } else { "No" });
