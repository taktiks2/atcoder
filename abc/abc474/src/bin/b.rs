// algo: simulation, grouping
use proconio::input;

// 座席番号 1..N を 10 人ずつに区切ったものがグループで、退出順 i 番目 (0-indexed) の人は
// 必ず第 i/10 グループに属していなければならない。つまり P_i が
// [10*(i/10) + 1, 10*(i/10) + 10] の範囲に入っているかを全 i について見れば判定できる。
// N <= 100 なので O(N)。
//
// 最後のグループが 10 人未満でも、上限を N より大きく取ったまま問題ない。P は 1..N の
// 順列なので N を超える値がそもそも現れず、範囲を広く取っても誤って Yes にはならない。

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    };

    let mut ans = true;

    for (i, &pi) in p.iter().enumerate() {
        let start = (i / 10) * 10 + 1;
        let end = start + 10;

        if !(start <= pi && end > pi) {
            ans = false;
            break;
        }
    }

    println!("{}", if ans { "Yes" } else { "No" });
}

// alt: 「P_i のグループ番号」と「i のグループ番号」の一致で書くと区間の上下端を組み立てずに済む。
//      all は false を見つけた時点で打ち切るので、mut フラグと break も消えて O(N) のまま
// let ans = p
//     .iter()
//     .enumerate()
//     .all(|(i, &pi)| (pi - 1) / 10 == i / 10);
// println!("{}", if ans { "Yes" } else { "No" });

// alt: Usize1 で 0-indexed に落として受け取れば pi - 1 が消える。座席番号を添字として
//      使う問題で毎回 -1 を書く事故を構造的に防げる
// use proconio::marker::Usize1;
// input! { n: usize, p: [Usize1; n] };
// let ans = p.iter().enumerate().all(|(i, &pi)| pi / 10 == i / 10);

// alt: chunks(10) で退出順を 10 人ずつに切ると「グループ番号 g の塊」がそのまま取れる。
//      i / 10 という添字計算が構造に置き換わり、問題文の「10 人ずつに分ける」に一番近い形
// let ans = p
//     .chunks(10)
//     .enumerate()
//     .all(|(g, group)| group.iter().all(|&pi| (pi - 1) / 10 == g));
// println!("{}", if ans { "Yes" } else { "No" });
