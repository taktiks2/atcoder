// algo: bfs, string
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// atcoder の並べ替えである S を、隣接 2 文字の交換だけで atcoder に戻す最小回数を求める問題。
//
// 「7 文字の並び 1 通り」を頂点、「隣接 2 文字の交換 1 回」を辺とみなすと、問われているのは
// S から "atcoder" への最短経路長になる。どの交換もコストが 1 で等しいので BFS で足りる
// (コストがばらつくならダイクストラが要る)。BFS が最短を返す根拠は「距離 k の頂点を全部
// 処理し終えてから距離 k+1 に進む」という処理順序だけで、グラフの形には一切依存しない。
// 逆に push_back/pop_front を pop_back (= DFS) に変えると壊れる。3 文字 cab -> abc で試すと
// acb がキューの底に取り残され、cab -> cba -> bca -> bac -> abc と遠回りした経路で先に
// ゴールに着き、正解 2 に対して 4 を返す。
//
// 隣接リストは作らない。作るには 5040 通りの並びに 0..5039 の通し番号を振る手間が要るが、
// Vec<char> をそのまま HashMap のキーにすれば「隣は誰か」を swap で都度計算できる。
// グリッドの BFS で隣接リストを持たず (i + di, j + dj) を計算するのと同じ形。
//
// 計算量は頂点が 7! = 5040、各頂点から出る辺が 6 本 (交換位置 i = 0..=5)、1 辺あたり
// Vec<char> の clone とハッシュ計算に O(7) かかるので 5040 * 6 * 7 ≈ 21 万。
// 型は答えが最大 7 * 6 / 2 = 21 (完全に逆順な redocta) なのでオーバーフローとは無縁。
//
// 落とし穴が三つ。
// 一つ目は dist の既出判定で、省くと cab -> acb -> cab と往復して永久に終わらない。
// 二つ目は交換位置の範囲で、0..cur.len() ではなく 0..cur.len() - 1。len まで回すと i + 1 が
// 範囲外になって panic (= RE)。
// 三つ目は #[fastout] と return の相性。fastout は関数本体を let res = { ... }; で包み、その
// 「後」に flush する展開なので (proconio-derive の doc comment を参照)、本体の途中で return
// すると flush されないまま main を抜けて出力が丸ごと消える。だから目的地に着いても return
// せず break し、println! は main の末尾に置いている。
// なお出力は 1 行なので #[fastout] 自体は本問では効いていない。

#[fastout]
fn main() {
    input! {
        s: Chars,
    };

    let target: Vec<char> = "atcoder".chars().collect();

    let mut dist: HashMap<Vec<char>, usize> = HashMap::new();
    let mut queue: VecDeque<Vec<char>> = VecDeque::new();
    dist.insert(s.clone(), 0);
    queue.push_back(s);

    let mut ans = 0;
    while let Some(cur) = queue.pop_front() {
        let d = dist[&cur];
        if cur == target {
            ans = d;
            break;
        }
        for i in 0..cur.len() - 1 {
            let mut next = cur.clone();
            next.swap(i, i + 1);
            if !dist.contains_key(&next) {
                dist.insert(next.clone(), d + 1);
                queue.push_back(next);
            }
        }
    }

    println!("{ans}");
}

// alt: 隣接交換 1 回で前後が入れ替わるペアはちょうど一組なので、答えは最初から順序が逆に
//      なっているペアの個数 (転倒数) にそのまま等しい。5040 頂点も HashMap も VecDeque も
//      要らず、O(N^2) = 49 の数え上げ 5 行で終わる。tuple_combinations が i < j のペア列挙を
//      そのまま表すので、式が転倒数の定義そのものになるのが良い。最短
// let a: Vec<usize> = s.iter().map(|&c| "atcoder".find(c).unwrap()).collect();
// let ans = (0..a.len())
//     .tuple_combinations::<(usize, usize)>()
//     .filter(|&(i, j)| a[i] > a[j])
//     .count();

// alt: 数えるのではなく実際に動かす版で、左から順に「i 番目に来るべき文字」を見つけて隣接
//      交換で運ぶだけ。これはバブルソートそのもの。運ぶ途中ですれ違う文字は必ず target[i]
//      より後ろに来るべき文字なので、1 回の交換が必ず 1 組の逆順を解消していて無駄が無く、
//      操作回数がそのまま最小になる。転倒数の考察をしなくても書けるうえ、BFS と違って状態を
//      持たないので追加メモリも要らない。input! を mut s: Chars にする
// let target: Vec<char> = "atcoder".chars().collect();
// let mut ans = 0;
// for i in 0..target.len() {
//     let mut j = (i..s.len()).find(|&j| s[j] == target[i]).unwrap();
//     while j > i {
//         s.swap(j - 1, j);
//         j -= 1;
//         ans += 1;
//     }
// }

// alt: BFS の骨格は変えず、状態を Vec<char> から固定長の [u8; 7] に置き換える版。配列は Copy
//      なので next を作るのに clone が要らず (let mut next = cur; で済む)、辺 3 万本ぶんの
//      ヒープ確保とハッシュ時のポインタ追跡が消えて定数倍が効く。長さが型に埋まるので
//      0..cur.len() - 1 の添字ミスも起こしようがない。状態が固定長のときの状態空間 BFS の定石
// let mut start = [0u8; 7];
// for (i, &c) in s.iter().enumerate() {
//     start[i] = "atcoder".find(c).unwrap() as u8;
// }
// let target: [u8; 7] = [0, 1, 2, 3, 4, 5, 6];
// let mut dist: HashMap<[u8; 7], usize> = HashMap::new();
// let mut queue: VecDeque<[u8; 7]> = VecDeque::new();
// dist.insert(start, 0);
// queue.push_back(start);
// let mut ans = 0;
// while let Some(cur) = queue.pop_front() {
//     let d = dist[&cur];
//     if cur == target {
//         ans = d;
//         break;
//     }
//     for i in 0..6 {
//         let mut next = cur;
//         next.swap(i, i + 1);
//         if !dist.contains_key(&next) {
//             dist.insert(next, d + 1);
//             queue.push_back(next);
//         }
//     }
// }
