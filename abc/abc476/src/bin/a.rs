// algo: string
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 末尾が 'e' なら 'r' を、そうでなければ 'er' を足すだけ。|S| <= 10 なので計算量は
// 何も気にしなくてよく、詰まるとすれば末尾の取り方だけ。
//
// s[s.len() - 1] が安全なのは |S| >= 1 が保証されているから。|S| = 0 があり得る制約なら
// s.len() - 1 は usize のアンダーフローで巨大な添字になり、範囲外で panic する。
// 空文字列を渡されうる問題では last() で Option にするか、長さを先に確かめる。
//
// Chars で受けているので末尾判定が添字アクセスになっているが、String で受ければ
// ends_with('e') と書ける (下の alt)。出力も join が要らなくなる。

#[fastout]
fn main() {
    input! {
        mut s: Chars,
    };

    let ans = if s[s.len() - 1] == 'e' {
        s.push('r')
    } else {
        s.push('e');
        s.push('r');
    };

    let ans = s.iter().join("");
    println!("{ans}");
}

// alt: String で受ければ ends_with で末尾を直接判定でき、Chars -> join の往復が丸ごと
//      消える。main の `let ans = if ... { s.push('r') } else { ... };` は両腕とも ()
//      を返すので ans: () になり、直後の let ans に影にされている (動作に害は無いが
//      意味の無い束縛)。分岐自体を値を返す式にすれば、この取り違えが起きようがなくなる
// input! { s: String };
// let ans = if s.ends_with('e') {
//     format!("{s}r")
// } else {
//     format!("{s}er")
// };
// println!("{ans}");

// alt: 分岐の中身を見ると 'r' は両方に共通で、違うのは 'e' を足すかどうかだけ。共通部分を
//      括り出すと分岐が片側だけになり、片方の push を書き漏らす形の間違いが構造的に
//      起きなくなる。この問題で一番短い
// input! { mut s: String };
// if !s.ends_with('e') {
//     s.push('e');
// }
// s.push('r');
// println!("{s}");
