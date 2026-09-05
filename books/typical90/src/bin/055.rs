// algo: brute-force, mod-arithmetic
use proconio::input;

// N <= 100 なので 5 個選ぶ組合せは C(100, 5) = 75,287,520 通り。5 重ループで i<j<k<l<m
// と絞れば順序無視・重複無しの列挙になり、積 mod P == Q を数えるだけで間に合う。
//
// A_i <= 10^9 を 5 個そのまま掛けると最悪 10^45 で usize (最大 1.8*10^19) を溢れる。
// 合同算術の性質 (a*b) mod P = ((a mod P) * (b mod P)) mod P を使い、掛けるたびに
// mod を取れば途中値は (P-1)^2 <= 10^18 に収まり usize で扱える。
//
// 部分積 factor を外側のループで確定させ、内側では 1 回の掛け算+剰余で伸ばしていく。
// 最深部で 5 個を一気に掛けるより、外側で使い回す分だけ演算量が大きく減る。

fn main() {
    input! {
        n: usize,
        p: usize,
        q: usize,
        a: [usize; n],
    };

    let mut cnt = 0;

    for i in 0..n {
        for j in i + 1..n {
            let factor = (a[i] * a[j]) % p;
            for k in j + 1..n {
                let factor = (factor * a[k]) % p;
                for l in k + 1..n {
                    let factor = (factor * a[l]) % p;
                    for m in l + 1..n {
                        let factor = (factor * a[m]) % p;
                        if factor == q {
                            cnt += 1
                        }
                    }
                }
            }
        }
    }

    println!("{cnt}");
}

// alt: 入力の a を最初に `a.iter().map(|&x| x % p).collect()` として mod 済みに揃えて
//      おくと、5 重ループ内の各掛け算に入る値が保証付きで (P-1) 以下になり、上限計算
//      (P-1)^2 が使える論拠が明確になる。P > max(A_i) のケースでは値は変わらないので、
//      現在のコードでも動作は同じ

// alt: itertools::Itertools::combinations を使えば「5 個選ぶ」意図をネストではなく型で
//      表現できる。部分積の巻き上げが効かず定数倍で遅くなるが、選ぶ個数 k を可変にした
//      Select k 拡張には自然
// use itertools::Itertools;
// let cnt = (0..n).combinations(5).filter(|v| {
//     v.iter().fold(1usize, |acc, &i| acc * a[i] % p) == q
// }).count();

// alt: 再帰 DFS で「残り何個選ぶか」と「次に選べる開始位置」を引数に取る書き方。5 に
//      固定なので今回はループ展開の方が読みやすいが、k を可変にしたい場合や
//      「prod == 0 かつ q != 0 なら打ち切り」等の剪定を挟みたい場合はこちらが素直
// fn dfs(a: &[usize], p: usize, q: usize, start: usize, rest: usize, prod: usize) -> usize {
//     if rest == 0 { return (prod == q) as usize; }
//     (start..=a.len() - rest)
//         .map(|i| dfs(a, p, q, i + 1, rest - 1, prod * a[i] % p))
//         .sum()
// }
// let ans = dfs(&a, p, q, 0, 5, 1 % p);

// alt: 中間 (meet in the middle) で 2+3 に分ける発想もあるが、P が素数と限らないため
//      T*X ≡ Q (mod P) を解く際に T の modular inverse が常に取れず、単純にはハッシュで
//      引けない。本問題では素直な 5 重ループの方が実装量・実行時間ともに有利
