// algo: string
// solve: self 2026-11-24
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 「空席で、左右の隣も空席 (または隣が存在しない)」椅子を数える問題。N <= 100 なので
// 各椅子について左右を見るだけの O(N) で十分で、計算量の工夫は要らない。論点は端の扱いだけ。
//
// 条件 2・3 は「隣が無い、または隣が x」。言い換えると「隣に o が居ない」で、範囲外は
// 空席と同じ扱いになる。|| の短絡評価がこの順番に効いていて、左辺 (隣が無い) が true なら
// 右辺の s[i - 1] / s[i + 1] は評価されない。順番を逆にして添字アクセスを先に書くと、
// i = 0 で i - 1 が usize のアンダーフロー、i = n - 1 で s[n] が範囲外になり panic (= RE)。
//
// 端を「条件を満たさない」側に倒すと WA になる。サンプル 1 (xxoxxxox) は左端の椅子 1 が
// 答えに入り、サンプル 3 (N = 1, "x") は左右とも隣が無い椅子が答え 1 になるので、
// 端の扱いはサンプルで検出できる。N = 1 では i = 0 が左端かつ右端で、両方の「隣が無い」が
// 同時に成り立つ。
//
// i.checked_sub(1).is_none() は i == 0 と同じ意味。i は usize なので i - 1 を先に計算して
// 比較する形 (i - 1 < 0 など) は書けず、「引く前に 0 かどうかを見る」必要がある。
// 答えは高々 N = 100 で型の心配は無い。出力は 1 行なので #[fastout] は本問では効いていない。

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    };

    let ans = s
        .iter()
        .enumerate()
        .filter(|&(i, c)| {
            *c == 'x'
                && (i.checked_sub(1).is_none() || s[i - 1] == 'x')
                && (i + 1 >= n || s[i + 1] == 'x')
        })
        .count();

    println!("{ans}");
}

// alt: リファクタリング版。構造はそのまま (各 i について「自分・左・右」の 3 条件を filter で
//      数える)。削ったのは次の 3 点
//      - i.checked_sub(1).is_none() -> i == 0 (同じ意味で、端の判定だと一目で分かる)
//      - i + 1 >= n -> i + 1 == n (i < n なので超えることは無く、== で「右端」を直接言える)
//      - iter().enumerate() の (i, c) -> 添字 i だけ。左右は s[i ± 1] で引くので自分も s[i] に
//        揃えると 3 条件が同じ形に並び、|&(i, c)| と *c の参照外しも消える
// let ans = (0..n)
//     .filter(|&i| s[i] == 'x' && (i == 0 || s[i - 1] == 'x') && (i + 1 == n || s[i + 1] == 'x'))
//     .count();

// alt: 条件を「x である」ではなく「o でない」と読み替えると、範囲外と空席が一つにまとまる。
//      slice::get は範囲外で None を返すので s.get(i) != Some(&'o') は「椅子が無い」でも
//      「空席」でも true。i = 0 の左隣は wrapping_sub(1) で usize::MAX になり、これも
//      範囲外として None に落ちる。|| による端の場合分けが消え、3 条件が完全に同じ形になる。
//      Rust らしい書き方
// let free = |i: usize| s.get(i) != Some(&'o');
// let ans = (0..n)
//     .filter(|&i| free(i.wrapping_sub(1)) && free(i) && free(i + 1))
//     .count();

// alt: 番兵。両端に空席 'x' を足してしまえば「隣が無い」というケース自体が無くなり、
//      問題は「長さ 3 の窓がすべて x である箇所の数」になる。windows(3) の中央が元の各椅子に
//      一対一で対応する (窓は N + 2 - 2 = N 個)。端の処理をデータ側に押し込む定番の手で、
//      添字計算が一切出てこないので境界バグの余地が無い。最短
// let mut t = vec!['x'];
// t.extend(&s);
// t.push('x');
// let ans = t.windows(3).filter(|w| w.iter().all(|&c| c == 'x')).count();
