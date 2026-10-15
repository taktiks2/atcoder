// algo: sliding-window
// solve: copy 2026-10-15
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// X と . の文字列で、. を K 個まで X に変えたときの X の最長連続長を求める問題。
// |S| <= 2×10^5 なので、区間 [l, r] を全部試す O(|S|^2) = 4×10^10 は TLE。
//
// X に変える . は「連続する K 個」に限ってよい。変えた . の間に変えなかった . が
// 一つでも挟まると、そこで X の連続が切れるので、離れた . に使った分は無駄になる。
// . の位置を並べた列で j 番目〜j+K-1 番目を変えると決めると、X の連続区間は
// 「窓の左隣の .」と「窓の右隣の .」に挟まれた部分になり、長さは
//   右隣の位置 - 左隣の位置 - 1
// で決まる。よって長さ K+2 の窓 (左隣 + 変える K 個 + 右隣) を滑らせて最大を取れば
// よく、O(|S|)。
//
// 端の処理は番兵で消している。左端の外 (位置 -1) と右端の外 (位置 n) にも . がある
// と思えば、端に接する区間も同じ式で書ける。usize で -1 を持てないので全体を +1 ずらし、
// 番兵を 0 と n+1 にしている。番兵なしで dot_indexes[k] - 1 のように書くと、
// "XXXX" / K=200000 (. が 0 個) で添字が範囲外の panic、".X" / K=0 で 0 - 1 の
// usize アンダーフローになる。
//
// unwrap_or(n) は「. の個数 m < K」の受け皿。dot_indexes の長さは m+2 なので、
// m < K だと windows(k + 2) は窓を 1 つも返さず max() が None になる。このときは全部
// X にできるので答えは n。m = K のときは窓が 1 つ ([0, ..., n+1]) で (n+1) - 0 - 1 = n
// が普通に出る。K = 0 なら windows(2) で隣り合う . の間の X の連続をそのまま見る形になる。
// 出力は 1 行なので #[fastout] は本問では効いていない。

#[fastout]
fn main() {
    input! {
        s: Chars,
        k: usize,
    };

    let n = s.len();

    // 左端に番兵の'.'を追加
    let mut dot_indexes: Vec<usize> = vec![0];
    for (i, &c) in s.iter().enumerate() {
        if c == '.' {
            dot_indexes.push(i + 1);
        }
    }
    // 右端に番兵の'.'を追加
    dot_indexes.push(n + 1);

    let ans = dot_indexes
        .windows(k + 2)
        .map(|w| w[k + 1] - w[0] - 1)
        .max()
        .unwrap_or(n);

    println!("{ans}");
}

// alt: リファクタリング版。構造はそのまま (番兵 2 つ + windows(k + 2))
//      - . の位置を集める for + if + push の 5 行 → itertools の positions + extend の 1 行
// let mut dot_indexes = vec![0];
// dot_indexes.extend(s.iter().positions(|&c| c == '.').map(|i| i + 1));
// dot_indexes.push(n + 1);
//
// let ans = dot_indexes
//     .windows(k + 2)
//     .map(|w| w[k + 1] - w[0] - 1)
//     .max()
//     .unwrap_or(n);

// alt: 尺取り法。. の位置の配列も番兵も要らず、S を直接 1 回なめるだけで O(|S|)。
//      「区間内の . が K 個以下」という条件の形がそのまま出るので、条件を
//      「区間和 <= K」などに変えた類題にもそのまま流用できる。m < K の場合分けも不要
// let mut ans = 0;
// let mut dots = 0; // 区間 [l, r] に含まれる . の個数
// let mut l = 0;
// for r in 0..n {
//     if s[r] == '.' {
//         dots += 1;
//     }
//     while dots > k {
//         if s[l] == '.' {
//             dots -= 1;
//         }
//         l += 1;
//     }
//     ans = ans.max(r + 1 - l);
// }
// println!("{ans}");
