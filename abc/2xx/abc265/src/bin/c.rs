// algo: simulation, grid
// solve: self 2026-10-18
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// (1,1) から矢印に従って進み、盤外に出る手前で止まる位置を答える (止まらないなら -1)。
// H, W <= 500 なのでマスは高々 2.5×10^5 個。
//
// 「いつまで回せば無限と言えるか」が本題。次のマスは今いるマスだけで決まる (決定的) ので、
// 一度訪れたマスに再び来たら、そこから先は前回と全く同じ道を辿り、また同じマスに戻って
// くる。つまり 2 度目の訪問 = 無限ループ確定。逆に止まる場合は同じマスを踏まないので、
// 移動は高々 HW - 1 回で終わる。どちらにせよ高々 HW + 1 ステップで決着し O(HW)。
// 「10^9 回回して止まらなければ -1」のような打ち切りは TLE するし、回数の根拠も無い。
// 鳩の巣で HW 回が上限と言えるのがこの問題の芯。
//
// ループ判定を「(1,1) に戻ってきたか」で書くのは誤り。経路は ρ 字型 (途中まで一本道で、
// その先の輪に入る) になり得て、輪が始点を含むとは限らない。sample2 は 6 マス全部が輪で
// 始点を含むため、この誤判定でもサンプル 3 つは全部通ってしまう。
// 「サンプルは通るのに WA (実際は TLE)」の典型形。
//
// 座標を i32 で持つのは U/L で -1 を作るため。cur = (0, 0) は h as i32 との比較から i32 に
// 推論されている。usize のまま cy - 1 を書くと行 0 で U を踏んだ瞬間に debug ビルドで
// panic (release では usize::MAX に wrap して ty >= h で弾かれるので、偶然動く)。
// dist は 1 次元 (y * w + x) に潰した訪問フラグで、値は 0 / 1 しか取らない。
// 止まったときに出すのは「出ようとした先」ではなく cur で、1-indexed に直して出力する。
// 出力は 1 行なので #[fastout] は本問では効いていない。

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        g: [Chars; h],
    };

    let mut dist: Vec<usize> = vec![0usize; h * w];
    let mut collision = false;
    let mut cur = (0, 0);
    dist[0] = 1;

    while !collision {
        let (cy, cx) = cur;

        let (ty, tx) = match g[cy as usize][cx as usize] {
            'U' => (cy - 1, cx),
            'D' => (cy + 1, cx),
            'L' => (cy, cx - 1),
            'R' => (cy, cx + 1),
            _ => unreachable!(),
        };

        if ty < 0 || ty >= h as i32 || tx < 0 || tx >= w as i32 {
            collision = true;
        } else {
            if dist[(ty * w as i32 + tx) as usize] > 0 {
                // 2度目に訪れたらループするのでその時点で無限判定
                println!("-1");
                return;
            } else {
                dist[(ty * w as i32 + tx) as usize] += 1;
                cur = (ty, tx);
            }
        }
    }

    println!("{} {}", cur.0 + 1, cur.1 + 1);
}

// alt: 構造はそのまま (ループ内で match → 次のマス → 盤外 / 既訪問の判定) で冗長な所だけ削った版
//      - dist (usize の += 1) → seen (bool)。値は 0/1 しか取らないので意図が型に出る
//      - collision フラグ → loop + break。フラグを立てて次の周回で抜ける遠回りが消える
//      - else { if .. } else { .. } の入れ子 → break / return で早期脱出して平らにする
//      - 添字 (ty * w as i32 + tx) as usize を 2 回書いていたのを id 1 回に
//      - cur = (0i32, 0i32) で型を明示 (比較からの推論に頼らない)
// let mut seen = vec![false; h * w];
// let mut cur = (0i32, 0i32);
// seen[0] = true;
//
// loop {
//     let (cy, cx) = cur;
//
//     let (ty, tx) = match g[cy as usize][cx as usize] {
//         'U' => (cy - 1, cx),
//         'D' => (cy + 1, cx),
//         'L' => (cy, cx - 1),
//         'R' => (cy, cx + 1),
//         _ => unreachable!(),
//     };
//
//     if ty < 0 || ty >= h as i32 || tx < 0 || tx >= w as i32 {
//         break;
//     }
//     let id = (ty * w as i32 + tx) as usize;
//     if seen[id] {
//         println!("-1");
//         return;
//     }
//     seen[id] = true;
//     cur = (ty, tx);
// }
//
// println!("{} {}", cur.0 + 1, cur.1 + 1);

// alt: usize のまま書く Rust らしい版。盤外を「次のマスが存在しない = None」と捉え、
//      checked_sub (0 - 1 で None) と filter (h, w 以上で None) で表す。i32 との往復キャストと
//      1 次元化が消え、2 次元 Vec をそのまま引ける。ループ条件自体が「未訪問の間」になり、
//      抜けたら -1、None なら止まった位置、と出口が 2 つに分かれて読みやすい
// let mut seen = vec![vec![false; w]; h];
// let (mut i, mut j) = (0usize, 0usize);
// while !seen[i][j] {
//     seen[i][j] = true;
//     let next = match g[i][j] {
//         'U' => i.checked_sub(1).map(|ni| (ni, j)),
//         'D' => Some((i + 1, j)).filter(|&(ni, _)| ni < h),
//         'L' => j.checked_sub(1).map(|nj| (i, nj)),
//         'R' => Some((i, j + 1)).filter(|&(_, nj)| nj < w),
//         _ => unreachable!(),
//     };
//     let Some((ni, nj)) = next else {
//         println!("{} {}", i + 1, j + 1);
//         return;
//     };
//     (i, j) = (ni, nj);
// }
// println!("-1");

// alt: 訪問配列すら要らない版。止まる経路は同じマスを踏まないので移動は高々 HW - 1 回、
//      つまり HW 回目の試行までに必ず「盤外に出ようとする」判定に当たる。HW 回動いても
//      止まらなければ鳩の巣でどこかを 2 度踏んでいる = 無限。上の考察の上限をそのまま
//      ループ回数にした形で、メモリ O(1)
// let (mut i, mut j) = (0i32, 0i32);
// for _ in 0..h * w {
//     let (ni, nj) = match g[i as usize][j as usize] {
//         'U' => (i - 1, j),
//         'D' => (i + 1, j),
//         'L' => (i, j - 1),
//         'R' => (i, j + 1),
//         _ => unreachable!(),
//     };
//     if ni < 0 || ni >= h as i32 || nj < 0 || nj >= w as i32 {
//         println!("{} {}", i + 1, j + 1);
//         return;
//     }
//     (i, j) = (ni, nj);
// }
// println!("-1");
