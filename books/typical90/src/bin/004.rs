// algo: prefix-sum
use proconio::input;

// 行和 + 列和 - 自分自身
//
// マス (i,j) の答えは「行 i の総和 + 列 j の総和」。ただし A[i][j] は行にも列にも入っていて
// 2 回数えているので、1 回分を引く。
//
// マスごとに十字を舐め直すと O(HW(H+W)) = 4*10^6 * 4000 で間に合わない。行和・列和を先に作れば
// 1 マスあたり O(1) で答えられるので、全体 O(HW) = 4*10^6 に落ちる。
//
// 行和と列和を同じ 2 重ループで同時に作っているのは、列和を (0..w).map(|j| a.iter().map(|row| row[j]))
// のように列ごとに集めると行をまたぐ飛び飛びのアクセスになり、キャッシュに乗らないため。
// 行優先で 1 回舐めれば、読んだ要素をそのまま row_sums[i] と col_sums[j] の両方に足し込める。

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    };

    let mut row_sums = vec![0; h];
    let mut col_sums = vec![0; w];

    for (i, row) in a.iter().enumerate() {
        for (j, ai) in row.iter().enumerate() {
            row_sums[i] += ai;
            col_sums[j] += ai;
        }
    }

    for (i, row) in a.iter().enumerate() {
        for (j, ai) in row.iter().enumerate() {
            print!(
                "{}{}",
                row_sums[i] + col_sums[j] - ai,
                if j + 1 == w { "" } else { " " }
            );
        }
        println!();
    }
}

// alt: 出力は最大 400 万マス (約 28MB)。print! はマスごとに stdout をロックして書式化するので、
//      002 と同じく String に溜めて最後に 1 回出す方が速い (BufWriter::new(stdout().lock()) に
//      writeln! しても同じ)。区切りを常に ' ' にすると行末に空白が残るが、AtCoder のジャッジは
//      行末の余分な空白を無視するので、j による分岐も落とせる
// use std::fmt::Write; // String に write! するために必要
// let mut out = String::with_capacity(h * w * 7);
// for (i, row) in a.iter().enumerate() {
//     for (j, ai) in row.iter().enumerate() {
//         write!(out, "{} ", row_sums[i] + col_sums[j] - ai).unwrap();
//     }
//     out.push('\n');
// }
// print!("{out}");

// alt: A[i][j] <= 99, H,W <= 2000 なので答えは高々 99 * (2000 + 2000) = 396000 で u32 に収まる。
//      グリッドは 400 万要素あるので、usize (8 byte) を u32 (4 byte) にするだけで 32MB -> 16MB。
//      走査するデータ量が半分になるぶんキャッシュにも効く
// a: [[u32; w]; h],
// let mut row_sums = vec![0u32; h];
// let mut col_sums = vec![0u32; w];

// alt: 行和は map、列和は fold で書くと可変変数が消え、「行を 1 本ずつ畳んでいく」構造が形に出る。
//      fold でも row を外側に回すので行優先アクセスは崩れない (代わりにグリッドを 2 周する)
// let row_sums: Vec<usize> = a.iter().map(|row| row.iter().sum()).collect();
// let col_sums = a.iter().fold(vec![0usize; w], |mut acc, row| {
//     for (j, ai) in row.iter().enumerate() {
//         acc[j] += ai;
//     }
//     acc
// });
