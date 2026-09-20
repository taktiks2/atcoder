// algo: tree, dp
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 各アメーバが根 (アメーバ 1) から何代目かを全頂点分出す問題。頂点は 1..=2N+1 で最大
// 400001、辺は「親 A_i -> 子 2i, 2i+1」の 2N 本で、全体は根付き木になる。
//
// 頂点ごとに親を根まで遡って数える愚直は落ちる。A_i に直前に生まれた子を選び続けると木は
// 一直線になり深さが N まで伸びるので、4×10^5 頂点 × 2×10^5 = 4×10^10 で TLE。遡るのを
// やめて、親が確定した時点で「親の世代 + 1」を子に配る向きにすると各頂点 O(1) になる。
//
// それが 1 パスで済む根拠が制約 A_i <= 2i-1、つまり「親の番号は必ず子より小さい」こと。
// 番号の昇順がそのままトポロジカル順になっているので、記録を i の昇順に処理するだけで
// 親は必ず確定済みになる。トポロジカルソートも BFS のキューも要らないのはこの制約が
// 訪問順をタダでくれているからで、裏を返せば親子の番号に大小関係が無い問題では
// この書き方は成立しない (下の alt 参照)。
//
// amebas は 0-indexed で、添字 k がアメーバ k+1 を表す。記録 i で push する 2 個は添字
// 2i-1, 2i = アメーバ 2i, 2i+1 にちょうど一致するので、push 順と出力順が同じになり、
// 世代を溜めてから出し直すループが要らない。amebas[parent] を読む時点で添字 0..=2i-2 は
// 埋まっているため範囲外にもならない。
//
// 型は usize でよい。世代は高々 N = 2×10^5、番号も 2N+1 で溢れる余地が無い。ai - 1 も
// A_i >= 1 の制約からアンダーフローしない。
//
// 落とし穴は再帰 DFS で書いた場合。上の一直線ケースで再帰が 2×10^5 段になり、release
// ビルドでも "thread 'main' has overflowed its stack" で abort する (手元で再現確認)。
// サンプルは N = 2, 4 で深さ 2 しか無いので、サンプルだけでは絶対に気づけない形。
//
// 出力は 2N+1 = 最大 400001 行あるので、#[fastout] は本問では効いている。

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut amebas = vec![0usize];
    println!("0");

    for ai in a {
        let parent = ai - 1;
        for _ in 0..2 {
            let generation = amebas[parent] + 1;
            amebas.push(generation);
            println!("{generation}");
        }
    }
}

// alt: A を Usize1 で読むと ai - 1 の -1 が消え、0-indexed への変換が入力側に一本化される。
//      伸びる長さは 2N+1 で確定しているので with_capacity で再確保も潰せる。同じ親から
//      生まれる 2 匹は世代が等しいから、内側の 2 周ループは 2 回の push と 1 回の println!
//      に畳める。ロジックは同じで、添字調整という間違えやすい一点が消える分だけ短い
// input! { n: usize, a: [Usize1; n] };
// let mut amebas = Vec::with_capacity(2 * n + 1);
// amebas.push(0usize);
// println!("0");
// for parent in a {
//     let generation = amebas[parent] + 1;
//     amebas.push(generation);
//     amebas.push(generation);
//     println!("{generation}\n{generation}");
// }

// alt: 隣接リストを作って根から BFS する教科書形 (AtCoder Tags でこの問題が bfs に分類
//      されているのはこの解法)。本問では隣接リストのメモリと出力ループが増えるだけで得は
//      無いが、「番号の昇順 = トポロジカル順」という前提が消えた瞬間に必要になるのはこちら。
//      親子の番号に大小が無い / 辺が双方向 / 始点が複数、のどれかが来ると上の実装は壊れる。
//      全辺の重みが 1 なので BFS の訪問順がそのまま最短距離 = 世代になる。親 -> 子 の有向辺
//      しか張らないので後戻りが起きず visited は不要 (無向で張るなら visited か親の除外が要る)
// let mut children = vec![Vec::new(); 2 * n + 2];
// for (idx, p) in a.into_iter().enumerate() {
//     let i = idx + 1; // 観察記録は 1-indexed
//     children[p].push(2 * i);
//     children[p].push(2 * i + 1);
// }
// let mut generation = vec![0usize; 2 * n + 2];
// let mut queue = VecDeque::from([1usize]);
// while let Some(v) = queue.pop_front() {
//     for &c in &children[v] {
//         generation[c] = generation[v] + 1;
//         queue.push_back(c);
//     }
// }
// for k in 1..=2 * n + 1 {
//     println!("{}", generation[k]);
// }
