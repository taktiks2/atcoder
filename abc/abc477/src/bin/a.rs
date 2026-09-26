// algo: simulation
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// B -> Y -> R -> B ... と巡回する信号の「次の色」を答える問題。入力は 3 通りしか
// ないので、対応表をそのまま書けば終わる。
//
// 注意するのは R の次が B に戻るところだけ。「B -> Y -> R」の並びだけを見て R の次を
// 書き忘れる、あるいは Y と R を取り違える形のミスがありうる。ただサンプル 3 つが
// 3 通りの入力を網羅しているので、今回はサンプルが通れば全ケース正しい。
//
// else に 'R' を任せているのは、制約で c が B / Y / R のどれかに限られるから。

#[fastout]
fn main() {
    input! {
        c: char,
    };

    let color = if c == 'B' {
        'Y'
    } else if c == 'Y' {
        'R'
    } else {
        'B'
    };

    println!("{color}");
}

// alt: if-else の連鎖を match にすると「入力 -> 出力」の対応表がそのまま 1 行ずつ並ぶ。
//      Rust らしく、3 通りの対応を一目で確認できる
// let color = match c {
//     'B' => 'Y',
//     'Y' => 'R',
//     _ => 'B',
// };

// alt: 巡回を「並び順の文字列と (i + 1) % 3」で表す版。色が増えても "BYR" を書き換える
//      だけで済み、R -> B の折り返しを % が自動で処理する。周期列の「次」を求める一般形
// let s = "BYR";
// let i = s.find(c).unwrap();
// let color = s.as_bytes()[(i + 1) % 3] as char;
