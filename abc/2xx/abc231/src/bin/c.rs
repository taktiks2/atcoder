// algo: sort, binary-search
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 各クエリ x_j について「A_i >= x_j を満たす i の個数」を答える問題。
// クエリごとに A を全走査すると O(NQ)。N = Q = 2×10^5 で 4×10^10 なので当然 TLE。
//
// A をソートすると「x_j 以上の要素」は必ず配列の末尾側に連続して並ぶ。つまり
//   a[i] < x_j が true ... true, false ... false
// という単調な形になり、true / false の境界さえ分かれば個数は引き算で出る。境界は
// 二分探索で O(log N)、全体 O(N log N + Q log N) ≈ 2×10^5 × 18 で余裕を持って通る。
// A のソートは一度だけで、クエリ間で使い回せるのがポイント（クエリごとに O(N) の
// 前処理をしたら意味がない）。
//
// partition_point が要求する前提は「述語が前半 true / 後半 false に分かれていること」
// だけで、ソート済みであることそのものではない。返り値は最初に false になる添字 =
// true の個数なので、「x_j 未満の個数」がそのまま手に入り、n から引けば答えになる。
//
// 述語に <= ではなく < を使うのは、x_j と等しい要素を false 側（= 数える側）に
// 残したいから。等号の寄せ方だけで lower_bound / upper_bound が入れ替わる:
//   x 以上の個数    -> n - partition_point(|&ai| ai <  x)
//   x より大きい個数 -> n - partition_point(|&ai| ai <= x)
// 重複が何個あっても、全要素が条件を満たす（0 が返る）場合も、一つも満たさない
// （n が返る）場合も、この式のまま破綻しない。
//
// 出力は Q = 最大 2×10^5 行。println! は行ごとに write を呼ぶので #[fastout] は必須。
// なお a は usize の Vec なので、安定性の要らない sort_unstable のほうがわずかに速い。

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
        x: [usize; q],
    };

    a.sort();

    for xj in x {
        // 述語が true になる要素数 = xj 未満の個数 = xj 以上になる最初の添字
        let ok = a.partition_point(|&ai| ai < xj);

        println!("{}", n - ok);
    }
}

// alt: partition_point の中身をめぐる式で自前実装した版。ok / ng を「条件 a[idx] >= xj を
//      満たす添字 / 満たさない添字」と定義し、隣り合うまで詰める。ng = -1 は「条件を満たす
//      要素が一つも無い」ケースを分岐なしで扱うための番兵で、その都合上 ok / ng は isize。
//      ok = n（範囲外）も番兵で、「全要素が条件を満たす」場合に対応する。
//      std に任せれば off-by-one も番兵も消えるので、実戦では partition_point 一択
// for xj in x {
//     let mut ok = n as isize; // 範囲外の番兵。全要素が xj 未満のとき答えは 0
//     let mut ng = -1isize; // 範囲外の番兵。全要素が xj 以上のとき答えは n
//
//     while (ok - ng).abs() > 1 {
//         let mid = (ok + ng) / 2;
//         if a[mid as usize] >= xj {
//             ok = mid;
//         } else {
//             ng = mid;
//         }
//     }
//
//     println!("{}", n as isize - ok);
// }

// alt: ここで a.binary_search(&xj) を使ってはいけない。重複要素があると「どの添字が返るか」が
//      保証されず、しかも見つからない場合だけ Err(挿入位置) という別の形で返ってくるため、
//      個数を数える用途には使えない。境界を求めたいときは常に partition_point

// alt: A を降順にソートすれば述語の向きが反転し、partition_point の返り値がそのまま答えになる。
//      n - ok の引き算が消える分だけ式の意味が読み取りやすい
// a.sort_unstable_by(|l, r| r.cmp(l));
// for xj in x {
//     println!("{}", a.partition_point(|&ai| ai >= xj));
// }

// alt: クエリもソートしてしまえば二分探索すら要らない（オフライン処理）。x を昇順に見ると
//      境界は右にしか動かないので、ポインタ j を持ち越す尺取りで j は全体で高々 n 回しか
//      進まず、ソート後は O(N + Q)。出力は元の順に戻す必要があるので添字を持ち回る
// let mut queries: Vec<(usize, usize)> = x.into_iter().enumerate().map(|(i, xi)| (xi, i)).collect();
// queries.sort_unstable();
// let mut ans = vec![0; q];
// let mut j = 0; // a[0..j] が xj 未満であることを保つ
// for (xi, idx) in queries {
//     while j < n && a[j] < xi {
//         j += 1;
//     }
//     ans[idx] = n - j;
// }
// for v in ans {
//     println!("{}", v);
// }
