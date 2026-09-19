// algo: set
use proconio::input;
use std::collections::HashSet;

// これまでに登録された ID を HashSet に貯め、初めて現れた行だけ番号を出す
//
// 素直に「i 番目の ID が i-1 番目までのどれかと一致するか」を二重ループで判定すると
// O(N^2 * L) = 10^10 * 12 で TLE。既登録集合を HashSet で持てば 1 件あたり平均 O(L)、
// 全体 O(N * L) で捌ける。
//
// HashSet::insert は「集合に無くて新規追加した場合に true」を返してくれるので、
// contains → insert の 2 段構えを取らずに済み、初出判定と登録が 1 回のハッシュ計算で
// 終わる。

fn main() {
    input! {
        n: usize,
        s: [String; n],
    };

    let mut set = HashSet::new();

    for (i, si) in s.iter().enumerate() {
        if set.insert(si) {
            println!("{}", i + 1);
        }
    }
}

// alt: enumerate は 0 始まりなので毎ループ +1 して 1-origin に直しているが、
//      (1..).zip(...) にすれば番号側を最初から 1 始まりで持てて足し算が消える
// for (i, si) in (1..).zip(s.iter()) {
//     if set.insert(si) {
//         println!("{i}");
//     }
// }

// alt: HashSet はハッシュキーが乱択化されているとはいえ、敵対的な入力では衝突で
//      最悪 O(N) に劣化しうる。BTreeSet に置き換えれば 1 挿入が O(L log N) に増えるが、
//      赤黒木で最悪計算量が保証される。返り値の意味 (新規なら true) は HashSet と同じ
// use std::collections::BTreeSet;
// let mut set: BTreeSet<&String> = BTreeSet::new();
