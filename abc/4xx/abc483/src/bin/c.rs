// algo: counting, sort
// solve: copy 2026-11-09
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 候補者 k が辞退したときの当選者を全 k について求める問題。k ごとに全投票を数え直して
// 全候補者から最大を探すと 1 回 O(N + M)、全体で 2×10^5 × 4×10^5 = 8×10^10 で TLE。
//
// k が辞退して票数が動くのは「第一希望が k の人の第二希望」だけで、それ以外の候補者の票数は
// k によらず第一希望の票数 cnt のまま。そこで当選者の候補を次の 2 種類に絞る:
//   - base: 票数が動かない側の 1 位。cnt で最初に 1 回ソートした order の先頭 (k 本人なら 2 番目)
//   - second[k] の人たち: 票が増えた側。辞退後の票数 cnt + add で比べる
// base 以外で票が増えていない人 c は cnt[c] <= cnt[base] (同点なら番号も base より大きい) で、
// base の票数は増えることはあっても減らないので、c が base に勝つことはない。よってこの候補だけ
// 比べれば十分。base 自身が second[k] に含まれていても cnt + add で比べるので正しい。
// second[k] の長さは k について合計 M なので、全体で O(N log N + M)。
//
// N >= 2 なので order[1] は必ず存在する。票数は高々 M = 2×10^5 で usize で十分。
//
// 落とし穴:
// - add を k ごとに 0 に戻し忘れると前の辞退者の分が残る。cnt を直接書き換えて戻さない実装も同じで、
//   サンプル 1 の k=2 で 1 ではなく 3 を出して WA
// - 同点は番号の小さい方が勝つ。sort と max_by_key の両方で Reverse の付け所を間違えると、
//   同点のケース (サンプル 1 の k=3 や k=5) で落ちる
// - 入力は Usize1 で 0-indexed にしているので、出力は winner + 1

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut ab: [(Usize1, Usize1); m],
    };

    let mut cnt = vec![0usize; n];
    let mut second = vec![vec![]; n];
    for &(a, b) in &ab {
        cnt[a] += 1;
        second[a].push(b);
    }

    let mut order: Vec<usize> = (0..n).collect();
    // sort_by_key: 各要素からキーを作り、キーの昇順に並べる
    // タプルは 1 要素目で比べ、同じなら 2 要素目で比べる。Reverse で大小を逆にできる
    // → (Reverse(票数), 番号) で「票数の降順、同点は番号の昇順」
    //   sort は昇順にしか並べないので、票数だけ逆順にしたいところに Reverse を付ける
    order.sort_by_key(|&c| (Reverse(cnt[c]), c));

    let mut add = vec![0usize; n];
    for k in 0..n {
        for &b in &second[k] {
            add[b] += 1;
        }

        let base = if order[0] != k { order[0] } else { order[1] };
        // once(x): x を 1 個だけ返すイテレータ
        // a.chain(b): a の後ろに b をつなげる → [base, 第二希望の人たち...]
        // max_by_key: キーが最大の要素を Option で返す (空なら None。base がいるので unwrap してよい)
        // → (辞退後の票数, Reverse(番号)) で「票数最大、同点は番号が小さい方」
        //   キーで比べるが、返すのはキーではなく元の要素 (番号)。欲しいのは「最大になる人」なので
        //   max() だと番号の最大、map(票数).max() だと誰の票数か分からなくなる
        //   max はキーが大きい方を選ぶので、番号をそのまま入れると同点で番号の大きい人が勝ってしまう。
        //   Reverse(番号) にすると番号が小さいほどキーが大きくなり、同点で番号の小さい人が勝つ
        //   型: 「〇〇が最大、同点なら△△が小さい方」を選ぶ → max_by_key(|x| (〇〇, Reverse(△△)))
        let winner = std::iter::once(base)
            .chain(second[k].iter().copied())
            .max_by_key(|&c| (cnt[c] + add[c], Reverse(c)))
            .unwrap();

        println!("{}", winner + 1);

        for &b in &second[k] {
            add[b] = 0;
        }
    }
}

// alt: 構造はそのまま (cnt / second を作り、order をソートし、k ごとに add を足して戻す)。
//      使っていない mut と参照を外した
//      - mut ab → ab (ab は書き換えていない)
//      - for &(a, b) in &ab → for (a, b) in ab (ab は後で使わないので所有権ごと回してよい)
// input! {
//     n: usize,
//     m: usize,
//     ab: [(Usize1, Usize1); m],
// };
// let mut cnt = vec![0usize; n];
// let mut second = vec![vec![]; n];
// for (a, b) in ab {
//     cnt[a] += 1;
//     second[a].push(b);
// }

// alt: 全候補者の (Reverse(票数), 番号) を BTreeSet で持ち、辞退のたびに票数が変わる人だけ
//      抜いて入れ直す。先頭 (first) がそのまま当選者なので、「動かない側の 1 位」と
//      「増えた側」を分ける考察が要らない。O((N + M) log N) で少し重いが、考えることが少ない
//      (cnt は mut で受ける。辞退者 k は抜いておき、最後に戻す)
// let mut set: BTreeSet<(Reverse<usize>, usize)> =
//     (0..n).map(|c| (Reverse(cnt[c]), c)).collect();
// for k in 0..n {
//     set.remove(&(Reverse(cnt[k]), k));
//     for &b in &second[k] {
//         set.remove(&(Reverse(cnt[b]), b));
//         cnt[b] += 1;
//         set.insert((Reverse(cnt[b]), b));
//     }
//     println!("{}", set.first().unwrap().1 + 1);
//     for &b in &second[k] {
//         set.remove(&(Reverse(cnt[b]), b));
//         cnt[b] -= 1;
//         set.insert((Reverse(cnt[b]), b));
//     }
//     set.insert((Reverse(cnt[k]), k));
// }
