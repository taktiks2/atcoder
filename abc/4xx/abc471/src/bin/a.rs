// algo: math
use proconio::input;

fn main() {
    input! {
        a: f32,
        b: f32,
    };

    const NINE: f32 = 9.0;

    let mut ans = false;

    if a + b == NINE || a - b == NINE || a * b == NINE || a / b == NINE {
        ans = true
    }

    println!("{}", if ans { "Nine" } else { "Nein" })
}

// alt: A/B == 9 を A == 9B に変形すれば整数のまま判定できる・float の == 比較を避けられる
// （整数除算 a / b == 9 だと 67/7 のような切り捨てで誤判定するので掛け算に寄せるのが安全）
// input! {
//     a: i32,
//     b: i32,
// }
// let ans = a + b == 9 || a - b == 9 || a * b == 9 || a == 9 * b;
// println!("{}", if ans { "Nine" } else { "Nein" });
