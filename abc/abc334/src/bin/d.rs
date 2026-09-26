// algo: greedy, sort, prefix-sum, binary-search
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// X 匹のトナカイで引けるソリの最大台数を、Q 個の X について答える問題。
//
// 台数を最大にしたいなら、必要数 R_i の小さいソリから順に取るのが最適。もし最適解に
// 取っていない小さいソリがあれば、取っている大きいソリと入れ替えても合計は増えず、
// 台数は変わらない。よって「小さい方から k 台」の形の最適解が必ずある。
// 答えは「R を昇順に並べた先頭 k 個の和 <= X」を満たす最大の k になる。
//
// クエリごとに先頭から足していくと 1 回 O(N) で、全体は N × Q = 4×10^10 になり TLE。
// ソートと累積和は X に依存しないので一度だけ作り、各クエリは累積和の二分探索
// O(log N) で答える。全体で O(N log N + Q log N)。
//
// prefix_sum[i] は「先頭 i+1 個の和」で、R_i >= 1 なので狭義単調増加。述語 x <= qi は
// 前半 true / 後半 false に分かれ、partition_point の返り値 (= true の個数) がそのまま
// 引ける台数になる。先頭に 0 を置かない形にしているので -1 の補正がいらない (下の alt 参照)。
// 比較を x < qi にすると、ちょうど X 匹で引けるケースを落とす。サンプル 2 の X = 1
// (R = 1 のソリ 1 台) がこれにあたり、1 ではなく 0 を出して WA になる。
//
// 型: 和は最大 2×10^5 × 10^9 = 2×10^14 で、i32 の 2.1×10^9 を大きく超える。
// X 自体も 2×10^14 まであるので、両方 64 bit の usize で持つ。
// Q = 2×10^5 行を出力するので #[fastout] が効く。

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        mut r: [usize; n],
        query: [usize; q],
    };

    r.sort_unstable();

    let prefix_sum: Vec<usize> = r
        .iter()
        .scan(0, |cum, x| {
            *cum += x;
            Some(*cum)
        })
        .collect();

    for qi in query {
        let ans = prefix_sum.partition_point(|&x| x <= qi);
        println!("{ans}");
    }
}

// alt: 先頭に 0 を置く累積和 (pre[i] = 先頭 i 個の和) の流儀で書く版。pre[0] = 0 <= X は
//      必ず成り立つので、返り値は「条件を満たす i の個数」= 最大の i + 1 になり、-1 で補正する。
//      区間和 pre[r] - pre[l] を取る問題ではこちらが標準なので、両方の添字の対応を押さえておく
// let mut pre = vec![0usize; n + 1];
// for i in 0..n {
//     pre[i + 1] = pre[i] + r[i];
// }
// for qi in query {
//     println!("{}", pre.partition_point(|&x| x <= qi) - 1);
// }

// alt: クエリを先読みして X の昇順に処理すれば、二分探索なしの尺取りで答えられる。X が
//      増えると引ける台数は減らないので、ポインタ k は戻らず、合計 N 回しか進まない。
//      計算量はクエリのソートで O(Q log Q) と変わらないが、「オフラインでクエリを並べ替える」
//      という別の定石が身につく。答えは元の順で出すので ans[i] に書き戻す
// let mut ord: Vec<usize> = (0..q).collect();
// ord.sort_unstable_by_key(|&i| query[i]);
// let mut ans = vec![0; q];
// let (mut k, mut sum) = (0, 0usize);
// for i in ord {
//     while k < n && sum + r[k] <= query[i] {
//         sum += r[k];
//         k += 1;
//     }
//     ans[i] = k;
// }
// for a in ans {
//     println!("{a}");
// }
