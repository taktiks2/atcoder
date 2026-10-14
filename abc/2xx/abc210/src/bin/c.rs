// algo: sliding-window, counting
// solve: self 2026-10-14
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 長さ K の連続区間のうち、含まれる色の種類数の最大値を求める問題。区間ごとに
// HashSet を作り直すと O(NK) で、N = 3×10^5, K = N/2 なら 4.5×10^10 で TLE。
//
// 隣の区間との差分は「左端の 1 個が抜け、右端の 1 個が入る」だけなので、色ごとの
// 個数を持ち回して差分更新すれば区間の移動は O(1)、全体で O(N)。種類数は
// 「個数が 0 -> 1 になった瞬間に +1、1 -> 0 になった瞬間に -1」で cur に追跡できる。
// 集合ではなく個数を持つのが要点で、同じ色が区間内に 2 個あるとき 1 個抜けても
// 種類数は減らない (集合で持つと抜けた瞬間に色ごと消してしまい WA)。
//
// c_i <= 10^9 なので色を添字にした Vec の度数表は張れない。HashMap で持つか、
// 座標圧縮して Vec にするか (下の alt 参照)。
//
// ans の更新を挿入時 (cur が増えたとき) だけにしているのは正しい。cur は挿入でしか
// 増えないので、最大値を取る瞬間は必ず挿入の直後にある。また「先に取り出す・後で
// 追加」の順なので、比較時点の cur は常にちょうど K 個の区間の種類数になっている。
// 逆順 (追加 -> ans 更新 -> 取り出し) にすると一瞬 K+1 個の区間で ans を更新して
// しまい 1 大きく出ることがある。しかもサンプル 3 つは逆順でも全部通る
// (N=3, K=1, c = 1 2 3 で 2 を出して落ちる)。「サンプルは通るのに WA」の典型形。
//
// 個数が 0 になったキーを remove しているので colors.len() が常に種類数と一致し、
// cur は実は不要 (下の alt 参照)。
// 出力は 1 行なので #[fastout] は本問では効いていない。

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        c: [usize; n],
    };

    let mut ans = 0usize;
    let mut cur = 0usize;
    let mut colors: HashMap<usize, usize> = HashMap::new();

    // 最初のウィンドウの色のマークとユニークな色の算出
    for &color in &c[..k] {
        if let Some(value) = colors.get_mut(&color) {
            *value += 1;
        } else {
            colors.insert(color, 1);
            cur += 1;
            ans = ans.max(cur);
        }
    }

    for i in k..n {
        let out_c = c[i - k];
        let in_c = c[i];

        // 先に取り出す
        if let Some(value) = colors.get_mut(&out_c) {
            *value -= 1;

            if *value == 0 {
                cur -= 1;
                colors.remove(&out_c);
            }
        }

        // 後で追加
        if let Some(value) = colors.get_mut(&in_c) {
            *value += 1;
        } else {
            colors.insert(in_c, 1);
            cur += 1;
            ans = ans.max(cur);
        }
    }

    println!("{ans}");
}

// alt: 構造はそのまま (初期区間のループ + 差分更新のループ、out_c / in_c の役割も同じ)。
//      - 挿入の if let / else 2 本 → entry().or_insert(0) の 1 行
//      - 取り出し側の if let は必ず Some なので unwrap で十分
//      - remove で 0 のキーを消しているので cur は colors.len() と常に一致 → cur を削除
//      - ans の更新は区間ごとに 1 回 (初期区間は colors.len() で初期化)
// let mut colors: HashMap<usize, usize> = HashMap::new();
// for &color in &c[..k] {
//     *colors.entry(color).or_insert(0) += 1;
// }
// let mut ans = colors.len();
// for i in k..n {
//     let out_c = c[i - k];
//     let in_c = c[i];
//
//     let value = colors.get_mut(&out_c).unwrap();
//     *value -= 1;
//     if *value == 0 {
//         colors.remove(&out_c);
//     }
//
//     *colors.entry(in_c).or_insert(0) += 1;
//     ans = ans.max(colors.len());
// }
// println!("{ans}");

// alt: 座標圧縮して Vec の度数表にする版。ハッシュ計算が消えて定数倍が速く、ループも
//      「i を入れて、i >= k なら i - k を抜く」の 1 本に畳める (初期区間の特別扱いが
//      不要)。i < k - 1 の間は K 未満の区間で ans を更新するが、それは最初の区間の
//      部分集合なので種類数は最初の区間以下で、最大値を汚さない
// let mut xs = c.clone();
// xs.sort_unstable();
// xs.dedup();
// let c: Vec<usize> = c.iter().map(|x| xs.binary_search(x).unwrap()).collect();
// let mut cnt = vec![0usize; xs.len()];
// let mut cur = 0;
// let mut ans = 0;
// for i in 0..n {
//     cnt[c[i]] += 1;
//     if cnt[c[i]] == 1 {
//         cur += 1;
//     }
//     if i >= k {
//         cnt[c[i - k]] -= 1;
//         if cnt[c[i - k]] == 0 {
//             cur -= 1;
//         }
//     }
//     ans = ans.max(cur);
// }
// println!("{ans}");
