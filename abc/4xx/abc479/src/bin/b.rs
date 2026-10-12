// algo: run-length
// solve: self 2026-10-12
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// R が連続する最大の長さを求める問題。今続いている R の長さ con を持ち、R なら +1、S なら
// 0 に戻す。毎回 ans = max(ans, con) を取るので O(N) の 1 周で済む。
//
// max を「S を見て run が切れたとき」だけ取ると、末尾まで R が続く run (サンプル 3 の
// 末尾 RRR など) を取り逃がす。この実装は文字ごとに max を取るので、末尾の後処理が要らない。
//
// R が一つも無いときは con が一度も増えず、ans = 0 がそのまま答えになる (サンプル 2)。
// n は読むだけで使っていない (Chars が長さを持っているため)。
// 出力は 1 行なので #[fastout] は本問では効いていない。

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    };

    let mut ans = 0usize;
    let mut con = 0usize;
    for si in s {
        if si == 'R' {
            con += 1;
        } else {
            con = 0usize;
        }

        ans = ans.max(con);
    }

    println!("{ans}");
}

// alt: 構造はそのまま。削ったのは:
//      - 0usize の型注釈 2 か所 (con = 0usize の再代入も含む)。推論で usize になる
// let mut ans = 0;
// let mut con = 0;
// for si in s {
//     if si == 'R' {
//         con += 1;
//     } else {
//         con = 0;
//     }
//     ans = ans.max(con);
// }

// alt: S で区切ると、残る断片がそのまま R の run になる。slice::split は区切りが
//      連続しても空の断片 (長さ 0) を返すので、全部 S でも max は 0 になり unwrap が安全
// let ans = s.split(|&c| c == 'S').map(|r| r.len()).max().unwrap();

// alt: itertools の dedup_with_count でランレングス圧縮して、R の run の長さの最大を取る。
//      「連続区間 = ランレングス」という見方がそのままコードになる。R が無いと空になる
//      ので unwrap_or(0)
// let ans = s
//     .iter()
//     .dedup_with_count()
//     .filter(|&(_, &c)| c == 'R')
//     .map(|(k, _)| k)
//     .max()
//     .unwrap_or(0);
