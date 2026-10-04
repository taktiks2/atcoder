// algo: simulation
use proconio::{fastout, input};

// 1..=N を順に見て、3 の倍数なら "Fizz"、そうでなければ i をそのまま出すだけ。O(N)。
// N <= 100 なので速度は問題にならないが、println! は 1 行ごとに標準出力のロックと
// フラッシュを取るので、出力行数が多い問題では #[fastout] (内部で BufWriter にまとめる)
// を付ける癖をつけておくと安全。
//
// if 式は両腕の型が一致していないといけないため、"Fizz" (&str) と i (usize) を
// そのまま並べることはできず、ここでは to_string() で String に揃えている。
// この「型を合わせるためだけの一時 String」を消すのが下の alt。

#[fastout]
fn main() {
    input! {
        n: usize,
    };

    for i in 1..=n {
        println!(
            "{}",
            if i % 3 == 0 {
                "Fizz".to_string()
            } else {
                i.to_string()
            }
        )
    }
}

// alt: 分岐の内側で println! を呼べば型を揃える必要がなくなり、String の確保も消える。
//      いちばん短くて速い書き方
// for i in 1..=n {
//     if i % 3 == 0 {
//         println!("Fizz");
//     } else {
//         println!("{i}");
//     }
// }

// alt: itertools::Either は両腕が Display なら自身も Display になるので、
//      「値を返す式」の形を保ったまま String 確保を避けられる。分岐した値を
//      そのまま map に流したいとき (型が違って collect できないとき) に効く
// use itertools::Either;
// for i in 1..=n {
//     let line = if i % 3 == 0 { Either::Left("Fizz") } else { Either::Right(i) };
//     println!("{line}");
// }

// alt: 出力を 1 本の String にまとめて一度だけ書き出す。#[fastout] を使わなくても
//      I/O が 1 回で済み、for が消えて宣言的になる (要 use itertools::Itertools)
// let ans = (1..=n)
//     .map(|i| if i % 3 == 0 { "Fizz".to_string() } else { i.to_string() })
//     .join("\n");
// println!("{ans}");
