// algo: math, gcd
use proconio::input;

fn gcd(a: usize, b: usize) -> usize {
    let rem = a % b;
    if rem == 0 {
        b
    } else {
        gcd(b, rem)
    }
}

// 一辺 gcd(A, B, C) の立方体に切り分ける
//
// 切り出す立方体の一辺を d とすると、3 辺のどれも d で割り切れないと余りが出るので、d は
// A, B, C の公約数でなければならない。そのときの切る回数は辺ごとの「分割数 - 1」の合計、
// つまり (A/d - 1) + (B/d - 1) + (C/d - 1) で、d を大きくするほど単調に減る。
// よって公約数のうち最大のもの = gcd(A, B, C) が最適で、公約数を全部試す必要はない。
//
// A, B, C <= 10^18 なので約数列挙 (O(sqrt(A)) = 10^9) すら重いが、ユークリッドの互除法なら
// O(log min(A, B))。gcd(gcd(A, B), C) と 2 回に分けてよいのは gcd が結合的だから。
//
// 型は入力の 10^18 と答えの最大 3*10^18 - 3 が乗る 64 bit が要る (i32 では溢れる)。

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    };

    let factor = gcd(a, b);
    let factor = gcd(c, factor);

    let ans: usize = [a, b, c].iter().map(|&x| x / factor - 1).sum();

    println!("{ans}");
}

// alt: AtCoder のジャッジには num が入っている (このパッケージの Cargo.toml にもある) ので、
//      gcd は自作せず num::integer::gcd を呼べば関数定義ごと消える。
//      こちらは b == 0 でも panic せず a を返す実装になっている
// use num::integer::gcd;
// let factor = gcd(gcd(a, b), c);

// alt: gcd(0, x) == x なので 0 を単位元にして fold できる。2 回の呼び出しを書き分けずに
//      「配列全体の gcd」という形にまとまり、辺が 4 本以上に増えても式が変わらない
// let factor = [a, b, c].iter().fold(0, |g, &x| gcd(g, x));

// alt: 互除法は「b が 0 になるまで (a, b) = (b, a % b)」のループでも書ける。再帰の深さは最悪
//      (連続するフィボナッチ数の入力) でも 10^18 なら 87 段程度なのでスタックは溢れないが、
//      ループ版は b == 0 が終了条件になるぶん、片方が 0 の入力 (今の実装は a % 0 で panic) も
//      そのまま吸収でき、上の fold と組み合わせても安全
// fn gcd(mut a: usize, mut b: usize) -> usize {
//     while b != 0 {
//         (a, b) = (b, a % b);
//     }
//     a
// }

// alt: 3 辺とも同じ factor で割るので、先に足してから 1 回だけ割ってもよい。除算が 3 回から
//      1 回に減る。A + B + C は最大 3*10^18 で usize (最大 1.8*10^19) に収まるが、i64
//      (最大 9.2*10^18) で持つ場合は「合計してから割る」形だと上限に近づく点に注意
// println!("{}", (a + b + c) / factor - 3);
