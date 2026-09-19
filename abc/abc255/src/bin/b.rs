// algo: brute-force
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 平面上の N 人のうち A に挙がった K 人が強さ R のライトを持ち、自分から距離 R 以内に
// いる人を照らす。全員がどれかのライトに照らされる最小の R を求める問題。
//
// 視点をライト側に置くと「どのライトが誰を担当するか」の割り当て問題に見えるが、ライトは
// 同時に光っていて互いに競合しない (一個が何人照らしてもよく、人 A の選択が人 B に影響
// しない) ので、割り当てを決める必要がそもそも無い。人 i は一番近いライトに照らして
// もらうのが常に最適で (遠いライトを選んでも R が増えるだけ)、要求する下限は
// min_j dist(i, j) に一意に決まる。全員が同時に満たされる必要があるから
//   R = max_i ( min_j dist(i, j) )
// になる。
//
// min と max の順序は入れ替えられない。max_i を先に取って min_j すると「一個のライト
// だけで全員を照らす」場合の答えになり、minimax 不等式 (min max >= max min) の通り
// 必ず過大な値が出る。ライトが互いに遠いほど差が開き、一次元に 0, 1, 5, 6 を並べて
// 両端をライトにすると正解 1 に対して 6 が出る。
//
// 計算量は O(NK)。N <= 1000, K < N なので距離計算は高々 10^6 回で二重ループのまま
// 間に合う。ライトを持たない人同士の距離は互いを照らせないので答えに関係せず、内側は
// ライトだけを回せばよい (全ペア O(N^2) は不要)。
//
// 距離は二乗のまま比較し、sqrt は確定後に一度だけ呼ぶ。sqrt は単調増加なので二乗でも
// 大小関係が保たれ、丸めの入る回数が 10^6 回から 1 回に減る。
//
// 型は i64。|X_i|, |Y_i| <= 10^5 で入力自体は i32 に収まるが、座標差は最大 2*10^5、
// その二乗和は (2*10^5)^2 * 2 = 8*10^10 となり i32 の上限 2.1*10^9 を超える。「入力が
// 小さいから i32 で足りる」と判断すると二乗の時点で溢れる形。座標が負を取るので usize
// も不可 (引き算でアンダーフローする)。
//
// A は 1-origin なので Usize1 で受ける。usize のまま読むと添字が 1 ずれ、A_K = N の
// ケースで p[n] を引いて RE になる。
//
// ライトを持つ人自身も照らされる対象なので外側は p 全体を回す。自分との距離 0 が min に
// 入るため、除外処理は要らない。
//
// 出力は絶対誤差・相対誤差 10^-5 まで許容。f64 の Display は往復可能な最短表記を出すので
// {} のままで桁が足りる。出力は 1 行なので #[fastout] は本問では効いていない。

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [Usize1; k],
        p: [(i64, i64); n],
    };

    let lights: Vec<(i64, i64)> = a.iter().map(|&ai| p[ai]).collect();

    let ans = p
        .iter()
        .map(|&(x, y)| {
            lights
                .iter()
                .map(|&(lx, ly)| {
                    let (tx, ty) = (lx - x, ly - y);
                    tx * tx + ty * ty
                })
                .min()
                .unwrap()
        })
        .max()
        .unwrap();

    println!("{}", (ans as f64).sqrt());
}

// alt: 「最小の R」を決め打ちして「その明るさで全員照らせるか」の判定問題に変えると、
//      min と max のネスト順を考えずに済む。各人について「距離 h 以内のライトが一個でも
//      あるか」を all/any で書くだけになり、本体で起こりうる max-min の取り違えが構造的に
//      起きない。h を増やすと照らされる人は増える一方なので判定は h について単調で、
//      小さい順に並べると NO...NO YES...YES になり、その境界が答え。lo は常に NO 側
//      (K < N かつ同座標の人がいないので h = 0 では必ず誰か照らされない)、hi は常に
//      YES 側 (|座標| <= 10^5 なので最遠でも 2*sqrt(2)*10^5 < 3*10^5) に取る。50 回で
//      幅は 3*10^5 / 2^50 になり許容誤差 10^-5 に十分。ただし判定 1 回が O(NK) なので
//      全体は O(NK log) と log 倍遅い。判定の中で全ライトとの距離を見ている以上その一周で
//      最小値も出てしまうため、本問では直接 max-min を取る方が速い。決め打ち二分探索は
//      「判定は簡単だが答えの直接計算が難しい」問題のための型で、本問はその退化形
// let ok = |h: f64| {
//     let hh = h * h;
//     p.iter().all(|&(x, y)| {
//         lights.iter().any(|&(lx, ly)| {
//             let (dx, dy) = ((lx - x) as f64, (ly - y) as f64);
//             dx * dx + dy * dy <= hh
//         })
//     })
// };
// let (mut lo, mut hi) = (0.0f64, 3.0e5f64);
// for _ in 0..50 {
//     let mid = (lo + hi) / 2.0;
//     if ok(mid) { hi = mid } else { lo = mid }
// }
// println!("{hi}");

// alt: lights の中間 Vec を作らず a から直接引き、pow(2) で tx/ty の一時束縛も消すと
//      max-min の骨格だけが残る。Vec の確保が一回減るが計算量は同じ O(NK) で、p[ai] の
//      二段引きが内側に入るぶん定数倍はわずかに重い。最短
// let ans = p
//     .iter()
//     .map(|&(x, y)| {
//         a.iter()
//             .map(|&ai| (p[ai].0 - x).pow(2) + (p[ai].1 - y).pow(2))
//             .min()
//             .unwrap()
//     })
//     .max()
//     .unwrap();
// println!("{}", (ans as f64).sqrt());

// alt: 座標を f64 で読んで hypot を使うと「ユークリッド距離」という意図がそのまま出て、
//      二乗のトリックも as f64 のキャストも消える。ただし f64 は Ord を実装しないので
//      min()/max() は使えず、fold(f64::INFINITY, f64::min) と fold(0.0, f64::max) に
//      なる (浮動小数を畳むときの Rust の定石。total_cmp を渡した min_by/max_by でも
//      よい)。本問の制約なら丸めが 10^6 回入っても誤差は許容内だが、比較が絡むうちは
//      整数で持つ方が安全。input! は p: [(f64, f64); n] にする。Rust らしさ
// let lights: Vec<(f64, f64)> = a.iter().map(|&ai| p[ai]).collect();
// let ans = p
//     .iter()
//     .map(|&(x, y)| {
//         lights
//             .iter()
//             .map(|&(lx, ly)| (lx - x).hypot(ly - y))
//             .fold(f64::INFINITY, f64::min)
//     })
//     .fold(0.0f64, f64::max);
// println!("{ans}");
