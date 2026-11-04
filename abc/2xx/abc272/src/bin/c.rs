// algo: parity, sort
// solve: self 2026-11-04
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 異なる 2 要素の和のうち、偶数になるものの最大値を求める問題。全ペアを見ると
// N(N-1)/2 で、N = 2×10^5 なら約 2×10^10 なので TLE。
//
// 和が偶数になるのは「偶数 + 偶数」か「奇数 + 奇数」のときだけで、偶数 + 奇数は必ず
// 奇数になる。なので A を偶奇で 2 グループに分ければ、各グループ内のどのペアの和も
// 偶数になり、グループをまたぐペアは考えなくてよい。グループ内で和を最大にするには
// 上位 2 つを取ればよい（A_i >= 0 なので、大きい方 2 つを選ぶのが常に最善）。
// 答えは「偶数グループの上位 2 つの和」と「奇数グループの上位 2 つの和」の大きい方。
//
// 降順ソートしてから partition するのがポイント。Iterator::partition は元の順序を
// 保ったまま振り分けるので、even と odd はそれぞれ降順のままになり、[0] と [1] が
// そのまま上位 2 つになる。計算量はソートの O(N log N)。
//
// 要素数が 1 以下のグループからはペアが作れない。両方 1 以下になるのは
// N = 2 で偶数 1 個・奇数 1 個のときだけで、そのとき -1。
//
// 落とし穴: 両グループに 2 個以上あるとき max を取り忘れて「偶数があれば偶数側」と
// 決め打ちする形。サンプル 1 は奇数が 3 の 1 個だけ、サンプル 2 は両方 1 個ずつで、
// 両グループが同時に成立するケースがサンプルに無いので、決め打ちでも全部通ってしまう。
// 例えば 2 4 5 7 なら偶数側 6 に対して奇数側 12 が答え。
//
// 型: 和の最大は 10^9 + (10^9 - 2) = 1999999998 で、i32 の上限 2147483647 には
// ぎりぎり収まるが余裕が無いので i64 で持つ。-1 を出すため符号付きにしておく。

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    };

    a.sort_unstable_by_key(|&x| Reverse(x));

    let (even, odd): (Vec<i64>, Vec<i64>) = a.into_iter().partition(|&x| x % 2 == 0);

    let ans = if even.len() > 1 && odd.len() > 1 {
        (even[0] + even[1]).max(odd[0] + odd[1])
    } else if even.len() > 1 {
        even[0] + even[1]
    } else if odd.len() > 1 {
        odd[0] + odd[1]
    } else {
        -1
    };

    println!("{ans}");
}

// alt: 構造はそのまま（降順ソート → partition → 上位 2 つの和）
//      - if/else if 4 本 → 「上位 2 つの和、無ければ -1」のクロージャ + max の 1 行
//      - 成立する和は必ず 0 以上なので、成立しない側を -1 にして max を取れば
//        「両方成立」「片方だけ」「どちらも不成立」が全部同じ式に落ちる
// a.sort_unstable_by_key(|&x| Reverse(x));
// let (even, odd): (Vec<i64>, Vec<i64>) = a.into_iter().partition(|&x| x % 2 == 0);
// let top2 = |v: &[i64]| if v.len() > 1 { v[0] + v[1] } else { -1 };
// let ans = top2(&even).max(top2(&odd));

// alt: ソート不要の O(N)。必要なのは各グループの上位 2 つだけなので、偶奇ごとに
//      [1 位, 2 位] を持って 1 回の走査で更新する。A_i >= 0 なので -1 を「空き」の
//      番兵にでき、t[1] >= 0 ならそのグループでペアが作れる。Vec の確保も消える
// let mut top = [[-1i64; 2]; 2]; // top[p] = 偶奇 p の [1 位, 2 位]
// for x in a {
//     let t = &mut top[(x % 2) as usize];
//     if x > t[0] {
//         t[1] = t[0];
//         t[0] = x;
//     } else if x > t[1] {
//         t[1] = x;
//     }
// }
// let ans = top
//     .iter()
//     .filter(|t| t[1] >= 0)
//     .map(|t| t[0] + t[1])
//     .max()
//     .unwrap_or(-1);

// alt: itertools の k_largest で「偶奇ごとの上位 2 つ」を直接取る。k_largest(2) は
//      全体をソートせず O(N log k) で上位 k 個を降順で返す。偶奇 2 通りを filter_map で
//      回し、ペアが作れない側を None に落とすので場合分けが消える。意図が一番素直に出る
// let ans = (0..2)
//     .filter_map(|p| {
//         let v = a.iter().filter(|&&x| x % 2 == p).k_largest(2).collect_vec();
//         (v.len() == 2).then(|| v[0] + v[1])
//     })
//     .max()
//     .unwrap_or(-1);
