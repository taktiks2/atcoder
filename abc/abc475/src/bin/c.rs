// algo: enumeration, prefix-sum
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 街は一直線に並んでいるので、街 j へ行くには S と j の間の街を必ず全部通る。つまり
// 訪問済みの集合は常に「S を含む連続区間 [l, r]」の形にしかならない。探索対象は区間だけ。
//
// 区間 [l, r] を決め打つと、必要な移動距離の最小値は閉じた式で書ける。
// X = (街 l と街 S の距離)、Y = (街 S と街 r の距離) として、
//   先に左端へ行く場合: S -> l -> r で 2X + Y
//   先に右端へ行く場合: S -> r -> l で X + 2Y
// 道を 3 回以上通るのは明らかに無駄なので、最小値は min(2X + Y, X + 2Y) で尽きている。
// 片側しか訪れない区間は X = 0 または Y = 0 になり、この式にそのまま含まれる。
//
// X, Y は座標の累積和 p を前計算すれば O(1) で引けるので、(l, r) の全探索で O(N^2)。
// N <= 8000 なので l <= S <= r の組は最大 8000^2 / 4 = 1.6×10^7 程度、余裕で間に合う。
//
// 「近い街から順に取る」貪欲は不正解。折り返し分を数えず距離を素朴に足してしまうため。
// 反例: N=3, S=2, L=2, A=(1,1) では貪欲は 1 + 1 = 2 <= 2 と判断して 3 街訪問と答えるが、
// 実際は 街1 -> 戻る -> 街3 で 3 の移動が要り L を超えるので答えは 2。
//
// 区間 DP (dp[l][r][今どちらの端にいるか] = 最小移動距離) としても定式化できるが、状態数が
// 8000 × 8000 × 2 ≈ 1.3 億で MLE。そもそもその DP の値が上の 2 式そのものなので、表を持つ
// 意味がない。制約の N <= 8000 が「状態を持たずに式にしろ」というヒントになっている。
//
// 座標の最大は 8000 × 10^9 = 8×10^12、2X + Y でも 2.4×10^13 なので u64 に収まる。
// L <= 10^18 との比較も問題ない。入力の L はループ変数 l と紛らわしいので limit と命名した。

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Usize1,
        limit: u64,
        a: [u64; n - 1],
    };

    // 街 0 を原点とした各街の座標
    let mut p = vec![0u64; n];
    for i in 0..n - 1 {
        p[i + 1] = p[i] + a[i];
    }

    let mut ans = 1;

    for l in 0..=s {
        let x = p[s] - p[l];

        for r in s..n {
            let y = p[r] - p[s];
            let cost = (2 * x + y).min(x + 2 * y);

            // cost は r について単調増加なので、超えた時点で内側は打ち切ってよい
            if cost > limit {
                break;
            }

            ans = ans.max(r - l + 1);
        }
    }

    println!("{ans}");
}

// alt: cost は X, Y の両方について単調増加なので「l を右に動かす (= X が縮む) ほど到達できる
//      r の上限は広義単調増加」。r を戻さない尺取り法にすると O(N) になる。
//      l が遠すぎて片道 (r = s、つまり左端へ行って終わり = cost X) すら limit を超える間は
//      区間自体が作れないので飛ばす。一度作れるようになれば l が増えるほど X は縮むだけ
//      なので、以降の l では必ず作れる。ここを 2 * x で判定すると折り返さない場合を
//      取りこぼして WA (反例: N=3, S=3, L=3, A=(1,3) で答え 2 を 1 と誤る)
// let mut r = s;
// for l in 0..=s {
//     let x = p[s] - p[l];
//     if x > limit {
//         continue;
//     }
//     while r + 1 < n {
//         let y = p[r + 1] - p[s];
//         if (2 * x + y).min(x + 2 * y) > limit {
//             break;
//         }
//         r += 1;
//     }
//     ans = ans.max(r - l + 1);
// }

// alt: 内側の線形探索を partition_point (C++ の lower_bound 相当) に置き換えると O(N log N)。
//      述語には「まだ許容範囲」= cost <= limit を渡すので、返るのは条件を満たす要素数、
//      つまり街 s から数えて何個右に伸ばせるかになる。尺取りより単調性の仮定が弱くて済む
// for l in 0..=s {
//     let x = p[s] - p[l];
//     let cnt = p[s..].partition_point(|&pr| {
//         let y = pr - p[s];
//         (2 * x + y).min(x + 2 * y) <= limit
//     });
//     if cnt > 0 {
//         ans = ans.max(cnt + (s - l));
//     }
// }

// alt: 座標の累積和は手書きループの代わりに scan + once で作れる。p[0] = 0 を先頭に連結し、
//      以降は直前の座標に A_i を足していく形。イテレータ側に寄せると mut が消える
// let p = std::iter::once(0u64)
//     .chain(a.iter().scan(0u64, |acc, &ai| {
//         *acc += ai;
//         Some(*acc)
//     }))
//     .collect_vec();

// alt: 「S から距離 d 以内で行ける街」を左右それぞれ二分探索し、折り返しの向きを 2 通り
//      試す書き方もある。この問題では区間の両端を直接回す方が短いが、「移動距離を決め打って
//      訪問数を求める」形なので、逆に「訪問数 K を決め打って最小移動距離が L 以下か」を
//      二分探索する問題 (答えを二分探索する典型) に変形することもできる
