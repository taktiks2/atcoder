// algo: string
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// T の '*' は「どんな小文字にもなれる 1 文字」なので、位置 i の判定は T[i] == '*' か
// T[i] == S[i] のどちらか。N <= 100 なので 1 文字ずつ見る O(N) で十分。
//
// 位置ごとの判定を全部満たせば全体も満たす、と言えるのがこの問題の肝。'*' は出現ごとに
// 別々の文字へ置き換えてよいので、ある位置の選択が他の位置の選択を縛らない。もし
// 「すべての '*' を同じ文字にする」という条件なら位置は独立でなくなり、この解法は壊れる。
//
// 正規表現の '*' (直前の文字の 0 回以上の繰り返し) と混同しないこと。本問の '*' は
// 1 文字にしか対応せず、S と T の長さはどちらも N で固定なので位置が 1 対 1 に決まる。
// 0 文字以上に対応すると思うと、長さの違う組を扱う全く別の問題 (DP が要る) になる。
//
// S は小文字のみで '*' を含まないので、S 側のワイルドカードを考える必要はない。

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
        t: Chars
    };

    let mut ans = "Yes";

    for i in 0..n {
        if t[i] == '*' {
            continue;
        }

        if t[i] == s[i] {
            continue;
        } else {
            ans = "No";
            break;
        }
    }

    println!("{ans}");
}

// alt: 早期 break は all の短絡評価がそのまま担うので、ループも添字も要らない。zip で
//      位置を突き合わせれば N すら使わなくなる (_n で読み捨て)。書き換える可変変数 ans が
//      消えて判定が式一つになり、break の入れ忘れで最後まで回るような壊れ方をしない
// input! { _n: usize, s: Chars, t: Chars };
// let ok = s.iter().zip(&t).all(|(&sc, &tc)| tc == '*' || tc == sc);
// println!("{}", if ok { "Yes" } else { "No" });

// alt: 問題文の定義「T の '*' を小文字に置き換えて S に一致させられるか」をそのまま
//      コードにした版。'*' を S の該当文字で埋めてから丸ごと比較する。Vec を 1 本作るぶん
//      main より重い (O(N) は同じ) が、問題文と実装の対応が一行ずつ追える
// let filled: Vec<char> = t
//     .iter()
//     .zip(&s)
//     .map(|(&tc, &sc)| if tc == '*' { sc } else { tc })
//     .collect();
// println!("{}", if filled == s { "Yes" } else { "No" });
