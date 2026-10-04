// algo: constructive
use proconio::input;

// 1..3 (= {1, 2}) から X を除くと必ず 1 個以上残るので、その先頭を出せばよい。
// この問題は「1 以上 3 以下で X と異なる整数」ならどれでも正解になる複数解問題。
// そのためサンプル出力 (2 → 3, 3 → 2) とは一致しないが、提出すれば AC になる。
// ローカルの `cargo compete test` は match: Lines で完全一致比較するので落ちる点に注意。

fn main() {
    input! {
        x: usize,
    };

    let ans: Vec<usize> = (1..3).filter(|&xi| xi != x).collect();
    println!("{}", ans[0]);
}

// alt: x % 3 + 1 なら分岐も探索も Vec 確保もなしで必ず X と異なる値になる
//      (1 → 2, 2 → 3, 3 → 1)。剰余で巡回させる構成は「自分以外を 1 つ選ぶ」系の定石
// println!("{}", x % 3 + 1);

// alt: find なら「条件を満たす最初の 1 個」を直接取れる。collect の Vec 確保と
//      ans[0] の添字アクセスが消え、問題文の言い回しにそのまま対応する
// println!("{}", (1..=3).find(|&xi| xi != x).unwrap());

// alt: 候補が 2 個しかないので単純な分岐でも書ける。X == 1 のときだけ 2、それ以外は 1
// println!("{}", if x == 1 { 2 } else { 1 });
