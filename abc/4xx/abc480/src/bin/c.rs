// algo: greedy, sort
// solve: hint 2026-10-18
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 0 秒から T 秒まで湯を切らさないために、他人の押下 A_i に加えて自分が最低何回押すかを求める問題。
// T, A_i <= 10^9 なので 1 秒ずつ追うシミュレーションは 10^9 ステップで論外。押下と押下の
// 「隙間」ごとに必要な回数を式で出す。ソートの O(N log N) が支配的。
//
// 自分が押すのは「湯が止まる瞬間」でよい（押下時刻と K 秒後ちょうども湯が出ている扱いなので、
// 止まる瞬間に押しても途切れない）。早めに押しても止まる時刻が手前にずれるだけで得をしない。
// 他人の押下はいつも、それまでのどの押下より後なので、止まる時刻を延ばすことはあっても縮めることは
// ない。だから押下どうしの隙間 d は互いに独立に考えられ、K 刻みで埋める回数は ceil(d/K) - 1。
// コードの `d / k`（割り切れたら -1）はこの値で、d <= K なら 0。境界の d == K は 0 回で足りるので
// 条件は `subi - k > 0` と strict にする。
//
// 0 秒ちょうどに誰も押していなければ、自分が 0 秒に押すしかない（i == 0 の +1）。
// 末尾は最後の押下から T までの隙間を同じ式で埋める。T 以降の押下は答えに関係ない。
//
// 落とし穴: 一番早い押下 a[0] が T より後のケース。sub[0] = a[0] だけはループの `t > ai` の
// 判定を通らないので、そのままだと関係ない隙間 0..a[0] を数えてしまう（入力 1 1 1 / 5 の答えは 1 なのに 5 になる）。
// このときは誰も押していないのと同じで、0..T を自分だけで埋めるので ceil(T/K) 回。ここで
// a[0] を使うと 1 1 3 / 5（答え 3）でも落ちる。サンプル 1, 2 はどちらもこの形を含まない。
//
// 値はすべて 10^9 以下で、答えも高々 ceil(T/K) + N 程度なので i32 でも溢れないが、差を取るので
// i64 にそろえておく。

#[fastout]
fn main() {
    input! {
        n: usize,
        k: i64,
        t: i64,
        mut a: [i64; n],
    };

    // 問題文でaが昇順だとは言及されてない
    a.sort_unstable();

    let mut sub: Vec<i64> = vec![0; n + 1];
    sub[0] = a[0];

    let mut ans = 0i64;
    if sub[0] > t {
        if t > k {
            // 青木くんが一度もボタンを押さずに高橋くんだけがボタンを押すパターン
            // 0秒目に押す1回目を足す
            ans += t / k + 1;
            if t % k == 0 {
                // 割り切れる場合は最後の1回が余分にカウントされるので、それを調整する
                ans -= 1;
            }
            println!("{ans}");
        } else {
            // 最初のボタン押下が目標秒数とKよりも少ないときは0秒目におすだけで終了
            println!("1");
        }
        return;
    }

    let mut last_idx = 0usize;
    for (i, &ai) in a[1..].iter().enumerate() {
        if t > ai {
            sub[i + 1] = ai - a[i];
            last_idx = i + 1;
        }
    }

    if t - a[last_idx] > k {
        sub[n] = t - a[last_idx];
    }

    for (i, &subi) in sub.iter().enumerate() {
        if i == 0 && subi != 0 {
            // 最初が0以外は必ず0秒目にボタンを押す
            ans += 1;
        }
        if subi - k > 0 {
            ans += subi / k;
            if subi % k == 0 {
                // 割り切れる場合は最後の1回が余分にカウントされるので、それを調整する
                ans -= 1;
            }
        }
    }

    println!("{ans}");
}

// alt: 構造はそのまま（隙間の配列 sub を作って、各隙間で ceil(d/K) - 1 を足す）。短くしたのは:
//      - a[0] > T の分岐 14 行 → `sub[0] = a[0].min(t)`。T で頭打ちにすると一般の式に乗る
//      - `d / k` + 割り切れたら -1 → `(d - 1) / k`（d >= 1 で ceil(d/K) - 1 と同じ値）
//      - enumerate のずれた添字 `sub[i + 1] = ai - a[i]` → `for i in 1..n` で揃える
//      - 末尾の `if t - a[last_idx] > k` → 無条件代入。d > k の判定は後のループが持つ
// a.sort_unstable();
// let mut sub = vec![0i64; n + 1];
// sub[0] = a[0].min(t); // T より後の押下は無関係なので T で頭打ち
// let mut last_idx = 0;
// for i in 1..n {
//     if a[i] < t {
//         sub[i] = a[i] - a[i - 1];
//         last_idx = i;
//     }
// }
// sub[n] = t - a[last_idx];
// let mut ans = if a[0] > 0 { 1 } else { 0 };
// for &d in &sub {
//     if d > k {
//         ans += (d - 1) / k;
//     }
// }
// println!("{ans}");

// alt: 配列を作らず「湯が止まる時刻 end」だけを持って前から貪欲に進める版（短さ・メモリ O(1)）。
//      0 秒の押下は自分でも他人でも end = K になるので、場合分けが ans の初期値だけに縮む。
//      x > end なら end から K 刻みで押して x に届くまで ceil((x - end)/K) 回、その後は x の押下で
//      end = x + K に更新する。take_while で T 以降を切るので、a[0] > T の特殊扱いも要らない
// a.sort_unstable();
// let mut ans = if a[0] > 0 { 1 } else { 0 };
// let mut end = k;
// for &x in a.iter().take_while(|&&x| x < t) {
//     if x > end {
//         ans += (x - end + k - 1) / k;
//     }
//     end = x + k;
// }
// if t > end {
//     ans += (t - end + k - 1) / k;
// }
// println!("{ans}");
