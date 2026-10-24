// algo: set
// solve: self 2026-10-24
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 名前ごとに ID を振り、既出ならその ID、初出なら新しい ID を出す問題。
// Q <= 2*10^5 なので、登録済みの名前を Vec に並べて毎回線形探索すると最悪
// 1 + 2 + ... + Q ≈ 2*10^10 回の比較で TLE。名前 -> ID の HashMap なら 1 クエリ
// 平均 O(|s|) (|s| <= 10) で、全体 O(Q * |s|) ≈ 2*10^6 で済む。
//
// 「使われていない最小の正整数」という言い方に身構えるが、口座は作るだけで消えないので、
// 振った ID は 1, 2, 3, ... と隙間なく埋まる。だから次の ID は常に「作った口座数 + 1」で、
// カウンタ id を 1 から増やすだけでよい (map.len() + 1 とも一致する)。削除がある問題なら
// 空き番号を BTreeSet などで管理する必要が出るが、本問では要らない。
//
// match map.get(&qi) の None 腕で map.insert できるのは、None 腕では get の借用を
// もう使っていないから (NLL)。Some(v) 腕で v を使いながら insert すると借用エラーになる。
//
// 出力は最大 2*10^5 行。#[fastout] が効く形なので println! をそのまま書いてよい。

#[fastout]
fn main() {
    input! {
        q: usize,
        query: [String; q],
    };

    let mut id = 1usize;

    let mut map: HashMap<String, usize> = HashMap::new();

    for qi in query {
        match map.get(&qi) {
            Some(v) => {
                println!("{v}");
            }
            None => {
                map.insert(qi, id);
                println!("{id}");
                id += 1;
            }
        }
    }
}

// alt: 構造はそのまま (カウンタ id と match get / insert)。
//      - map の型注釈を外した (insert(qi, id) から推論される)
//      - id の usize サフィックスを外した (最大 2*10^5 なので推論の i32 で足りる)
//      - 1 文だけの Some 腕の {} を外した
// let mut id = 1;
// let mut map = HashMap::new();
// for qi in query {
//     match map.get(&qi) {
//         Some(v) => println!("{v}"),
//         None => {
//             map.insert(qi, id);
//             println!("{id}");
//             id += 1;
//         }
//     }
// }

// alt: entry API で「既出なら取得、無ければ挿入」をハッシュ計算 1 回にまとめる版。
//      get → insert だと初出の名前で 2 回ハッシュを引くが、entry なら 1 回。次の ID は
//      map.len() + 1 で出せるのでカウンタ変数も要らない (削除が無いので len = 発行済み数)。
//      len は entry で可変借用する前に取っておく (or_insert(map.len() + 1) は E0502)
// let mut map = HashMap::new();
// for qi in query {
//     let next = map.len() + 1;
//     let v = *map.entry(qi).or_insert(next);
//     println!("{v}");
// }
