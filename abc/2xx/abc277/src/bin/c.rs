// algo: graph, bfs, connected-components
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// 1 階から出発してはしご (無向辺) を伝って行ける階のうち、最大の階番号を答える問題。
// 求めるものは「1 を含む連結成分の中の最大の頂点」で、到達集合を BFS で作って max を
// 取れば終わる。同じ階の中は自由に動けるという設定は、階を頂点に潰してよいという意味。
//
// 罠は頂点数の方。階は 10^9 まであるので vec![vec![]; 10^9] の隣接リストは論外 (空の
// Vec 一つで 24 バイト、10^9 個で 24 GB)。一方ではしごは N <= 2×10^5 本しか無いので、
// 実際に現れる階は多くても 2N = 4×10^5 種類しかない。グラフは極端に疎なので、階番号を
// そのままキーにした HashMap で持てば触れた階の分しかメモリを食わない。連続領域が
// 欲しければ座標圧縮して Vec に落とす (下の alt)。
//
// はしごは双方向なので隣接リストも両方向に張る。「一度下ってから別のはしごで登る」
// 経路を残すため。片方向だけにしてもサンプルは 3 つとも通ってしまうので注意で、例えば
// 2 本 (1, 5), (10, 5) なら答えは 10 なのに片方向だと 5 になる。
//
// visits へは「キューに入れる時点で」入れているので同じ頂点が二度キューに入らず、辺は
// 各方向 1 回ずつしか見ない。BFS 全体で O(N) で、頂点も辺も 4×10^5 程度なので余裕。
// 距離は使わないので DFS でも同じ (VecDeque をスタックに変えるだけで通る)。
//
// visits を始点入り ({0}) で初期化してあるのがポイント。1 階に繋がるはしごが 1 本も
// 無いとき (サンプル 3) も集合は空にならず、答えは 1 階自身の 1 になる。空集合から始めて
// 訪問時に入れる書き方にすると、この入力で max().unwrap() が panic して RE になる。
//
// 型は usize のまま。階番号は 10^9 以下、加算も最後の +1 だけなのでオーバーフローしない。
// Usize1 で 0-indexed にしている分を出力で +1 して戻す。
//
// graphs.entry(ai).or_insert_with(|| vec![bi]).push(bi) は、エントリが空のときだけ bi を
// 二度入れる (多重辺になる)。visits で弾くので答えは変わらないが、意図通りに書くなら
// or_default().push(bi) (下の alt)。
//
// 出力は 1 行なので #[fastout] は本問では効いていない。

#[fastout]
fn main() {
    input! {
        n: usize,
        ladders: [(Usize1, Usize1); n],
    };

    let mut graphs: HashMap<usize, Vec<usize>> = HashMap::new();

    for (ai, bi) in ladders {
        graphs.entry(ai).or_insert_with(|| vec![bi]).push(bi);
        graphs.entry(bi).or_insert_with(|| vec![ai]).push(ai);
    }

    let mut queue: VecDeque<usize> = VecDeque::from(vec![0]);
    let mut visits: HashSet<usize> = HashSet::from([0]);

    while let Some(start) = queue.pop_front() {
        if let Some(dists) = graphs.get(&start) {
            for &end in dists {
                if !visits.contains(&end) {
                    queue.push_back(end);
                    visits.insert(end);
                }
            }
        }
    }

    println!("{}", visits.iter().max().unwrap() + 1);
}

// alt: HashMap の entry は or_default().push() が定石で、多重辺が混ざらない。さらに
//      HashSet::insert は「新規なら true」を返すので contains + insert の 2 回の
//      ハッシュ計算が 1 回になり、最大値も pop のたびに畳めば最後の iter().max() が
//      消える。Rust らしさと定数倍の両取り
// let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
// for &(a, b) in &ladders {
//     graph.entry(a).or_default().push(b);
//     graph.entry(b).or_default().push(a);
// }
// let mut queue = VecDeque::from([0]);
// let mut visited: HashSet<usize> = HashSet::from([0]);
// let mut ans = 0;
// while let Some(v) = queue.pop_front() {
//     ans = ans.max(v);
//     for &u in graph.get(&v).map(|x| x.as_slice()).unwrap_or(&[]) {
//         if visited.insert(u) {
//             queue.push_back(u);
//         }
//     }
// }
// println!("{}", ans + 1);

// alt: 疎な頂点番号のもう一つの定石、座標圧縮で Vec<Vec<usize>> に落とす版。sort +
//      dedup した配列への partition_point が「階番号 -> 添字」の変換で、以降はハッシュ
//      計算が消えて訪問済みも Vec<bool> になる。ソートの分 O(N log N) に増えるが、
//      ハッシュが消えた分と相殺して実測はほぼ同じ (N = 2×10^5 の最大ケースでどちらも
//      30 ms 前後)。HashMap のキーが巨大でも困らない、という保険の形
// let mut xs: Vec<usize> = ladders.iter().flat_map(|&(a, b)| [a, b]).chain([0]).collect();
// xs.sort_unstable();
// xs.dedup();
// let id = |x: usize| xs.partition_point(|&y| y < x);
// let mut graph = vec![vec![]; xs.len()];
// for &(a, b) in &ladders {
//     let (a, b) = (id(a), id(b));
//     graph[a].push(b);
//     graph[b].push(a);
// }
// let s = id(0);
// let mut visited = vec![false; xs.len()];
// visited[s] = true;
// let mut queue = VecDeque::from([s]);
// let mut ans = 0;
// while let Some(v) = queue.pop_front() {
//     ans = ans.max(xs[v]);
//     for &u in &graph[v] {
//         if !visited[u] {
//             visited[u] = true;
//             queue.push_back(u);
//         }
//     }
// }
// println!("{}", ans + 1);

// alt: 使っているのは連結性だけなので、探索そのものを Union-Find に置き換えられる。辺を
//      全部 merge してから、1 階と同じ代表を持つ頂点のうち最大を取る。xs は昇順なので
//      後ろから最初に見つかったものが答えで、キューも訪問済み配列も要らない。最短
// use ac_library::Dsu;
// let mut xs: Vec<usize> = ladders.iter().flat_map(|&(a, b)| [a, b]).chain([0]).collect();
// xs.sort_unstable();
// xs.dedup();
// let id = |x: usize| xs.partition_point(|&y| y < x);
// let mut dsu = Dsu::new(xs.len());
// for &(a, b) in &ladders {
//     dsu.merge(id(a), id(b));
// }
// let r = dsu.leader(id(0));
// let i = (0..xs.len()).rev().find(|&i| dsu.leader(i) == r).unwrap();
// println!("{}", xs[i] + 1);
