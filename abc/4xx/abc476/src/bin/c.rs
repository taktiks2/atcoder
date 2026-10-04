// algo: sort
#![allow(unused_imports)]
use itertools::Itertools;
use petgraph::visit::Reversed;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// A_1..A_k を降順に並べた 3 番目を、k = 3, 4, ..., N の各時点で答える問題。k ごとに
// Vec を sort し直すと O(N^2 log N) で、N = 5×10^5 では論外。欲しいのは「1 個増えた」
// という差分だけを処理して整列状態を持ち越すことなので、挿入 O(log N) で順序を保つ
// BTreeSet を使う。N 回の挿入で O(N log N)、上位 3 件の取り出しは木の右端へ降りるだけ
// なので 1 回 O(log N)。全体を舐めないのが効いている。
//
// ただし BTreeSet は集合なので、値をそのまま入れると等しい値が 1 個に潰れる。この問題は
// 「19 が 2 個あれば降順の 1 番目と 2 番目は両方 19」なので、要るのは集合ではなく多重集合。
// 添字 i は 0..N で必ず相異なるから、(a[i], i) の形で入れれば N 個すべてが別要素として
// 残り、多重集合の代わりになる。タプルの Ord は辞書式なので並び順は第 1 要素 (値) で決まり、
// 添字は同値どうしの順序を決めるだけ。出力するのは .0 なので答えには影響しない。
//
// 潰したまま出すと二通りに壊れる。相異なる値が 3 個未満の k では取り出しが None になって
// RE、3 個以上あっても消えた重複のぶん順位がずれて WA (sample3 の k = 8 は 19 が 2 個で、
// 正解 17 に対し 11 を出す)。RE の方だけ「3 個無ければ 2 番目」のフォールバックで塞ぐと
// panic は消えて WA だけが残るので、症状ではなく多重集合にする側を直す。
//
// take(3).last() は「3 件あれば 3 番目、無ければ 2 番目、1 件なら 1 番目」。本問は
// i >= 2 の時点で必ず 3 要素以上あるので nth(2) でも通り、走査量もどちらも高々 3 要素。
//
// なお全要素を持つ必要は無い (公式解説)。k の上位 3 件は「k-1 の上位 3 件 + A_k」の
// 4 個の中にしか現れないので、それ以外は捨てても答えは変わらない。4 個から 3 個を選ぶ
// だけなら 1 ステップ O(1) で全体 O(N)、メモリも定数に落ちる (下の alt)。この main は
// 全部持つぶん O(N log N)・メモリ O(N) だが、N = 5×10^5 なら間に合う。
//
// A_i <= 10^9 で加減算もしないので usize のままオーバーフローしない。
// 出力が N-2 ≈ 5×10^5 行あるので #[fastout] は必須。

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut set = BTreeSet::new();

    set.insert((a[0], 0));
    set.insert((a[1], 1));

    for i in 2..n {
        set.insert((a[i], i));

        let third = set.iter().rev().take(3).last().unwrap().0;

        println!("{third}");
    }
}

// alt: 公式解説。上位 3 件だけ持ち、A_k を足した 4 要素を毎回ソートして末尾を捨てる。
//      4 要素のソートは O(1) なので全体 O(N)、メモリも定数。多重集合を作る必要が無い
//      (Vec は最初から重複を持てる) ので (値, 添字) の詰め物も BTreeSet も丸ごと消え、
//      この問題で一番短く書ける形
// let mut s = a[..3].to_vec();
// s.sort_unstable_by_key(|&x| Reverse(x));
// println!("{}", s[2]);
// for k in 3..n {
//     s.push(a[k]);
//     s.sort_unstable_by_key(|&x| Reverse(x));
//     s.pop();
//     println!("{}", s[2]);
// }

// alt: 同じ「上位 3 件だけ残す」を BTreeSet のまま実現する版。main の (値, 添字) を
//      活かしたまま O(N)・メモリ定数になる。set が 3 要素に固定されるので最小値がそのまま
//      降順 3 番目になり、rev().take(3).last() の合成が first() 一つに畳める
// for i in 0..n {
//     set.insert((a[i], i));
//     if set.len() > 3 {
//         set.pop_first();
//     }
//     if i >= 2 {
//         println!("{}", set.first().unwrap().0);
//     }
// }

// alt: BinaryHeap は最初から多重集合なので詰め物が要らず、3 を K に変えるだけで解説の
//      Bonus (K 番目への一般化) に届く。Reverse で最小ヒープにして K 件を超えたら pop
//      すれば残るのは上位 K 件で、peek がその最小 = 降順 K 番目。1 ステップ O(log K) で
//      全体 O(N log K)。ソートし直す alt は K が大きいと O(NK log K) になるので、
//      一般化するならこちらが本筋
// let k = 3usize;
// let mut heap = BinaryHeap::new();
// for i in 0..n {
//     heap.push(Reverse(a[i]));
//     if heap.len() > k {
//         heap.pop();
//     }
//     if i + 1 >= k {
//         println!("{}", heap.peek().unwrap().0);
//     }
// }
