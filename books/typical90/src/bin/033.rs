// algo: math, constructive
use proconio::input;

// 任意の 2×2 窓に点灯 LED は高々 1 個。盤面を重ならない 2×2 タイル (端は余ってよい)
// で敷き詰めるとタイル数は ceil(H/2) * ceil(W/2) 個で、これが点灯数の上限になる。
// 実際に「行も列も偶数」のマスだけ点ける配置はどの 2×2 窓にも高々 1 個しか入らず、
// この上限を達成できるので、これが最大値。
//
// ただし H=1 または W=1 のときは盤面に 2×2 窓自体が収まらず制約が全く効かないので、
// H*W 個すべて点灯できる。上の公式では ceil(H/2)*ceil(W/2) = 1*ceil(W/2) と過小
// 評価になる (H=1, W=5 なら公式 3, 正解 5) ため、この場合だけ分けて H*W を返す。
//
// f64 で受けているのは (h / 2.0).ceil() を直接使うため。H, W <= 100 なので仮数部
// 53 bit の精度に余裕があり、整数値のまま丸め誤差なく扱える。

fn main() {
    input! {
        h: f64,
        w: f64,
    };

    if h == 1.0 || w == 1.0 {
        return println!("{}", h * w);
    }

    let height = (h / 2.0).ceil();
    let width = (w / 2.0).ceil();

    println!("{}", height * width);
}

// alt: 整数のまま扱うなら usize で受けて div_ceil (Rust 1.73+) を使う。f64 の丸め誤差
//      を気にする必要がなく、大きな入力サイズにも自然に拡張できる
// input! { h: usize, w: usize };
// let ans = if h == 1 || w == 1 {
//     h * w
// } else {
//     h.div_ceil(2) * w.div_ceil(2)
// };
// println!("{ans}");

// alt: div_ceil を使わずとも、正の整数なら (h + 1) / 2 で ceil(h/2) と同じ値になる。
//      古い toolchain でも動く
// let ans = if h == 1 || w == 1 {
//     h * w
// } else {
//     ((h + 1) / 2) * ((w + 1) / 2)
// };

// alt: 例外条件は h.min(w) == 1 とも書ける。分岐が「短い辺が 1 か」という幾何的な
//      意味に寄るので、なぜ例外扱いなのか (2×2 窓が盤面に入らないから) が読みやすい
// if h.min(w) == 1.0 {
//     return println!("{}", h * w);
// }
