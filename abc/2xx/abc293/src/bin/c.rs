// algo: brute-force, grid
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// (1,1) から (H,W) へ右か下にだけ進む経路のうち、通ったマスの値がすべて異なるものを数える問題。
// 経路は H+W-2 歩で、そのうちどの H-1 歩を「下」にするかで決まるので C(H+W-2, H-1) 通り。
// H, W <= 10 なら最大でも C(18, 9) = 48620 通りしかなく、全経路を試して各経路で
// 18 マスの重複を調べても 10^6 程度。小さいグリッドの経路数え上げは全探索でよい。
//
// 経路の列挙は「i 歩目が右か下か」を bit の i ビット目に割り当てるビット全探索。
// 2^(H+W-2) = 2^18 = 262144 通りのうち経路として成立するのは 1 が W-1 個のものだけだが、
// 歩数は H+W-2 で固定なので、グリッドから一度もはみ出さずに歩き切れることと
// 「右がちょうど W-1 回・下がちょうど H-1 回」は同値。はみ出した時点で ng にして捨てれば、
// 残るのは (H-1, W-1) に着く正しい経路だけになる (終点の判定は念押しで、無くても結果は同じ)。
// 不成立の bit も途中までは歩くので計算量は 2^18 * 18 ≈ 4.7 * 10^6 で、それでも十分速い。
//
// 重複判定は値 -> 出現回数の HashMap で、全部 1 なら OK。A は 10^9 以下で配列の添字には
// できないので、値をキーにする集合系が要る。始点 a[0][0] は歩く前に入れておくこと
// (ループは 1 歩目から積むので、入れ忘れると始点と同じ値のマスを通る経路を誤って数える)。
//
// 落とし穴: 行 (下に進んだ回数 ti) と列 (右に進んだ回数 tj) の取り違え。範囲判定を
// tj < h && ti < w、参照を a[tj][ti] と書いても、H == W なら転置した盤面を歩くだけで
// 経路の集合も重複の有無も変わらず答えが一致する。サンプルは 3×3 と 10×10 の正方形しか無いので
// 全部通るのに、H != W のテストで WA になる (例えば H=3, W=2 だと ti が 2 まで進めず
// 常に 0 を出す)。実際この形で一度 WA を出した。
// 出力は 1 行なので #[fastout] は本問では効いていない。

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    };

    let pattern = h + w - 2;
    let mut cnt = 0;

    for bit in 0..1 << pattern {
        let mut map: HashMap<usize, usize> = HashMap::from([(a[0][0], 1)]);
        let mut ti = 0usize;
        let mut tj = 0usize;
        let mut ng = false;
        for i in 0..pattern {
            if bit >> i & 1 == 1 {
                // 右
                tj += 1;
            } else {
                // 下
                ti += 1;
            }
            if ti < h && tj < w {
                *map.entry(a[ti][tj]).or_insert(0) += 1;
            } else {
                ng = true;
                break;
            }
        }
        if !ng && ti == h - 1 && tj == w - 1 && map.values().all(|&x| x == 1) {
            cnt += 1;
        }
    }

    println!("{cnt}");
}

// alt: 再帰の DFS でマスを一つずつ進め、重複した瞬間に打ち切る版。HashSet::insert が
//      「新しく入ったか」を bool で返すのでそれがそのまま重複判定になり、帰りがけに remove して
//      集合を戻す (バックトラック)。グリッドの外へ出る分岐や右と下の個数が合わない bit を
//      そもそも生成しないので、訪れるのは成立する経路の接頭辞だけ。手元の 10×10 で
//      90 ms -> 4 ms。経路の形が素直にコードになり、行と列の取り違えも起きにくい
// fn dfs(a: &Vec<Vec<usize>>, i: usize, j: usize, seen: &mut HashSet<usize>) -> usize {
//     if !seen.insert(a[i][j]) {
//         return 0;
//     }
//     let (h, w) = (a.len(), a[0].len());
//     let res = if i == h - 1 && j == w - 1 {
//         1
//     } else {
//         let mut s = 0;
//         if i + 1 < h {
//             s += dfs(a, i + 1, j, seen);
//         }
//         if j + 1 < w {
//             s += dfs(a, i, j + 1, seen);
//         }
//         s
//     };
//     seen.remove(&a[i][j]);
//     res
// }
// // main では
// println!("{}", dfs(&a, 0, 0, &mut HashSet::new()));

// alt: 「H+W-2 歩のうちどの H-1 歩を下にするか」を itertools の combinations で直接列挙する版。
//      生成されるのは C(H+W-2, H-1) 個の正しい経路だけなので、はみ出し判定 ng も終点の判定も
//      要らない。all + insert で重複が出た瞬間に打ち切れ、filter().count() で
//      「条件を満たす経路の個数」がそのまま式になる
// let n = h + w - 2;
// let ans = (0..n)
//     .combinations(h - 1)
//     .filter(|downs| {
//         let (mut i, mut j) = (0, 0);
//         let mut seen = HashSet::from([a[0][0]]);
//         (0..n).all(|k| {
//             if downs.contains(&k) {
//                 i += 1;
//             } else {
//                 j += 1;
//             }
//             seen.insert(a[i][j])
//         })
//     })
//     .count();
// println!("{ans}");

// alt: AC 版と同じビット全探索のまま、先に count_ones() == W-1 で成立する bit だけに絞る版。
//      右の回数が W-1 なら下は自動的に H-1 回で必ず (H-1, W-1) に着くので、範囲外チェックと
//      ng フラグと終点判定が全部消える。HashMap の出現回数を最後に見る代わりに
//      HashSet::insert の bool を all で畳めば、重複した時点で打ち切れる
// let n = h + w - 2;
// let ans = (0usize..1 << n)
//     .filter(|bit| bit.count_ones() as usize == w - 1)
//     .filter(|bit| {
//         let (mut i, mut j) = (0, 0);
//         let mut seen = HashSet::from([a[0][0]]);
//         (0..n).all(|k| {
//             if bit >> k & 1 == 1 {
//                 j += 1;
//             } else {
//                 i += 1;
//             }
//             seen.insert(a[i][j])
//         })
//     })
//     .count();
// println!("{ans}");
