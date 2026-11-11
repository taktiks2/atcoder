// algo: string
// solve: self 2026-11-11
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 0 と 1 だけからなる文字列 s (長さ 1〜10) の 0 と 1 を入れ替える問題。
// 1 文字ずつ見て、反対の文字に写せば終わり。O(|s|)。
//
// 落とし穴: s.replace('0', "1").replace('1', "0") と置換を 2 回続けると、1 回目で全部 1 になり、
// 2 回目で全部 0 になる (サンプルが 3 つとも WA)。置換は同時に行わないと入れ替えにならない。
// replace でやるなら、一度 0 を別の文字に逃がす 3 段階が必要になる。map で 1 文字ずつ写せば
// 各文字を 1 回しか見ないので、この問題は起きない。
//
// s は 0 と 1 しか含まない制約なので、「0 でなければ 1」として else 側をまとめてよい。

#[fastout]
fn main() {
    input! {
        s: Chars,
    };

    let ans: Vec<char> = s
        .iter()
        .map(|&x| if x == '0' { '1' } else { '0' })
        .collect();

    println!("{}", ans.iter().join(""));
}

// alt: リファクタリング版。構造はそのまま
//      - Vec<char> に collect してから join("") → String に直接 collect。中間の Vec と join が消える
// let ans: String = s
//     .iter()
//     .map(|&x| if x == '0' { '1' } else { '0' })
//     .collect();
//
// println!("{ans}");

// alt: バイトの XOR で反転する。'0' = 0x30、'1' = 0x31 で最下位ビットだけが違うので、b ^ 1 で入れ替わる。
//      分岐が消えて短い。入力を marker::Bytes で受ける (use に Bytes の追加が必要)
// input! { s: Bytes };
// let ans: Vec<u8> = s.iter().map(|&b| b ^ 1).collect();
// println!("{}", String::from_utf8(ans).unwrap());
