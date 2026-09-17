// algo: sort, binary-search
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// A と B から一つずつ選んで |A_i - B_j| を最小化する問題。全ペアを試すと O(NM) で、
// N = M = 2×10^5 なら 4×10^10 なので当然 TLE。
//
// 片方 (B) をソートすると「A_i に一番近い B_j」の候補が二つに絞れる。B が昇順なら
// A_i 以上で最小の要素と A_i 未満で最大の要素が A_i を挟む形になり、そこから外側の
// 要素は必ず内側の要素より遠い。つまり境界の左右一個ずつを見れば十分で、境界の位置は
// 二分探索で O(log M)。全体で O(M log M + N log M) ≈ 2×10^5 × 18 に落ちる。
// B のソートは一度だけで A 全体に使い回すのがポイント (A_i ごとにソートしたら
// O(NM log M) で元より悪い)。
//
// 述語 (b_i - a_i) < 0 は b_i < a_i のことで、partition_point が返すのは
// 「a_i 未満の要素数」=「a_i 以上になる最初の添字」= lower_bound。よって i を挟んで
//   b[i - 1] < a_i <= b[i]
// が成り立ち、候補はこの二つだけになる。残りは端の処理で、i == 0 は「A_i が B の
// 全要素以下」、i == m は「A_i が B の全要素より大きい」で、それぞれ片側の候補が
// 存在しない。この二つを忘れると添字が範囲外に出て panic (= RE) になる。
//
// min を「毎回上書き」ではなく min.min(..) で畳むこと。上書きにすると出力は
// 「最後の A_i に対する最小値」になり、サンプル 3 つは偶然すべて最後の A_i が
// 最小差を作るので全部通ってしまう (sample1 は A=6 -> 4、sample3 は A=70 -> 67)。
// 「サンプルは通るのに WA」の典型形。
//
// 初期値 usize::MAX は N >= 1, M >= 1 の制約からループが必ず一周するので必ず上書き
// される。ただし min を usize で持つ都合で比較のたびに as usize が必要になっていて、
// isize のまま持てばキャストは全部消える (下の alt 参照)。
//
// 型は isize。値自体は 10^9 以下だが b_i - a_i が負になるので符号付きが必須で、
// usize で引くとアンダーフローして巨大な値になり abs も効かない。
// 出力は 1 行なので #[fastout] は本問では効いていない。

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [isize; n],
        mut b: [isize; m],
    };

    b.sort_unstable();

    let mut min = usize::MAX;

    for ai in a {
        // b[i - 1] < ai <= b[i] となる境界 (lower_bound)
        let i = b.partition_point(|&bi| (bi - ai) < 0);
        if i == 0 {
            min = min.min((b[i] - ai).abs() as usize) as usize;
        } else if i == m {
            min = min.min((b[i - 1] - ai).abs() as usize);
        } else {
            min = min.min((b[i] - ai).abs().min((b[i - 1] - ai).abs()) as usize);
        }
    }

    println!("{min}");
}

// alt: 3 分岐の正体は「候補の窓 b[i-1..=i] が配列の端で切れること」なので、窓を
//      saturating_sub と min でクリップすれば分岐そのものが消える。i == 0 なら b[0..1]、
//      i == m なら b[m-1..m]、それ以外は b[i-1..i+1] に自然に落ちる。min を isize で
//      持てるので as usize も消え、端の見落としが構造的に起きなくなる。最短
// b.sort_unstable();
// let ans = a
//     .iter()
//     .flat_map(|&ai| {
//         let i = b.partition_point(|&bi| bi < ai);
//         b[i.saturating_sub(1)..(i + 1).min(m)]
//             .iter()
//             .map(move |&bi| (bi - ai).abs())
//     })
//     .min()
//     .unwrap();
// println!("{ans}");

// alt: B を BTreeSet に入れれば range で前後を直接引ける。range(..ai).next_back() が
//      「ai 未満の最大」、range(ai..).next() が「ai 以上の最小」で、存在しない端は
//      None になるため端の場合分けが Option に吸収される。重複は勝手に潰れるが最小値には
//      影響しない。構築 O(M log M)・各クエリ O(log M) で計算量は同じ (定数倍はソート済み
//      Vec より重い)。「境界の前後を取る」という意図が最も素直に出る書き方
// let bs: BTreeSet<isize> = b.into_iter().collect();
// let ans = a
//     .iter()
//     .flat_map(|&ai| {
//         bs.range(..ai)
//             .next_back()
//             .into_iter()
//             .chain(bs.range(ai..).next())
//             .map(move |&bi| (bi - ai).abs())
//     })
//     .min()
//     .unwrap();

// alt: A もソートしてしまえば二分探索が要らない。両方昇順なら、今見ているペアから差を
//      縮める手は「小さい側のポインタを進める」しか無いので、マージのように一度ずつ
//      走るだけで最小差に当たる。ポインタは合計で高々 N + M 回しか進まず、ソート後は
//      O(N + M) で log が落ちる (入力は mut a にする)。片方だけソートする発想の一歩先
// a.sort_unstable();
// b.sort_unstable();
// let (mut i, mut j) = (0, 0);
// let mut ans = isize::MAX;
// while i < n && j < m {
//     ans = ans.min((a[i] - b[j]).abs());
//     if a[i] < b[j] {
//         i += 1;
//     } else {
//         j += 1;
//     }
// }

// alt: 二つの列を混ぜて一本にソートすると、答えは「隣接する異出身ペアの差の最小」になる。
//      最小を作るペアの間に要素が挟まっていたら、その要素と組み直しても差は増えないので、
//      最小を作るペアは必ず隣接まで詰められる。タプルの二要素目に出身 (0 = A, 1 = B) を
//      持たせれば同値も (5, 0) < (5, 1) で隣接し、差 0 を取りこぼさない。N, M >= 1 なので
//      境界は必ず一箇所以上あり unwrap は安全。O((N + M) log(N + M)) で計算量は同等だが、
//      二分探索も端の場合分けも消えて式一つになる
// let mut v: Vec<(isize, u8)> = a
//     .iter()
//     .map(|&x| (x, 0))
//     .chain(b.iter().map(|&x| (x, 1)))
//     .collect();
// v.sort_unstable();
// let ans = v
//     .windows(2)
//     .filter(|w| w[0].1 != w[1].1)
//     .map(|w| w[1].0 - w[0].0)
//     .min()
//     .unwrap();
