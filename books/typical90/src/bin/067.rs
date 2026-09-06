// algo: radix-conversion, string
use proconio::input;

// 入力文字列 s を 8 進表記とみなして u64 に変換し、その値を 9 進表記へ直しながら
// 桁「8」を「5」に置き換える。これを K 回繰り返す。1 回あたり O(len(s))、len(s) は
// 高々 20 桁 (N < 8^20)、K <= 100 なので全体でも余裕。
//
// N の値自体は 8^20 - 1 ≈ 1.15×10^18 で u64 に収まるが、入力に現れる「桁列」を
// 10 進の usize としてパースしようとすると、20 桁の 77...7 が u64::MAX を超えて
// panic する (このケースが実測で 3 ケース RE を引き起こしていた)。そのため必ず
// String で受け、以降も文字列として保持する。

fn main() {
    input! {
        n: String,
        k: usize,
    };
    let mut s = n;

    for _ in 0..k {
        // 8 進表記 → 数値 (u64)
        let mut decimal: u64 = s
            .chars()
            .rev()
            .enumerate()
            .map(|(i, c)| {
                let int = c.to_digit(10).unwrap() as u64;
                let base = 8u64.pow(i as u32);
                int * base
            })
            .sum();

        // 数値 → 9 進表記、桁「8」は「5」に置換
        let mut octal = String::from("");
        while decimal >= 9 {
            let result = (decimal / 9, decimal % 9);
            decimal = result.0;
            octal.push(if result.1 == 8 {
                '5'
            } else {
                char::from_digit(result.1 as u32, 10).unwrap()
            });
        }
        octal.push(if decimal == 8 {
            '5'
        } else {
            char::from_digit(decimal as u32, 10).unwrap()
        });
        s = octal.chars().rev().collect();
    }

    println!("{s}");
}

// alt: Rust 定石で書き直した版。以下 4 点で本体より短く速い:
//   - Horner 法 (fold) で pow + enumerate + rev を消す
//   - bytes() でバイト列を直接扱い、b'0' + d as u8 で char 変換の Option 剥きを省く
//   - loop { ... if v == 0 { break; } } の do-while で「while + 最後の push」を統合
//     (v == 0 の case も自然に処理)
//   - Vec<u8> + reverse() で chars().rev().collect::<String>() の UTF-8 デコードを回避
// use proconio::input;
//
// fn main() {
//     input! { n: String, k: usize }
//     let mut s = n;
//     for _ in 0..k {
//         let mut v: u64 = s.bytes().fold(0, |acc, b| acc * 8 + (b - b'0') as u64);
//         let mut buf = Vec::new();
//         loop {
//             let d = v % 9;
//             buf.push(if d == 8 { b'5' } else { b'0' + d as u8 });
//             v /= 9;
//             if v == 0 { break; }
//         }
//         buf.reverse();
//         s = String::from_utf8(buf).unwrap();
//     }
//     println!("{s}");
// }

// alt: 入力を u128 で受ければ 20 桁の 10 進整数もパースできるので数値のまま扱える。
//      ただし毎ループ to_string() が要り、String 保持と比べて特に得はない
// input! { n: u128, k: usize };
// let mut s = n.to_string();
