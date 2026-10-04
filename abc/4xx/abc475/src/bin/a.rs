// algo: string
use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};

// 「隣り合う文字の間にそれぞれ o を挿入」は「文字を区切り文字 o で連結する」と読み替えられる。
// S の長さを L とすると挿入箇所は L - 1 個で、先頭と末尾には付かない。join のセパレータの
// 入り方 (要素と要素の間だけ) がこの仕様とちょうど一致するので、場合分けも添字計算も要らない。
// L <= 10 なので計算量は考えなくてよい。
//
// サンプル 3 の "oo" -> "ooo" が効いていて、「元から o が隣にある位置は挿入しない」といった
// 解釈をすると落ちる。隣接ペアごとに無条件で 1 個ずつ挿入する。
//
// join は Itertools のもので、要素が Display なら String へ連結してくれる。標準ライブラリ側の
// slice::join は要素が &str 相当 (Borrow<str>) のときしか使えず、Vec<char> には直接刺さらない。

#[fastout]
fn main() {
    input! {
        s: Chars,
    };

    let ans = s.iter().join("o");

    println!("{ans}");
}

// alt: itertools の intersperse なら「要素の間に挟む」という操作そのものを名前で書ける。
//      join と違って char のまま流れるので Display 経由の文字列化が挟まらない。
//      ただし std にも同名の unstable な Iterator::intersperse があり、コンパイル時に
//      unstable_name_collisions 警告が出る (stable では itertools 側が選ばれるので動作は同じ)
// let ans: String = s.iter().copied().intersperse('o').collect();

// alt: itertools を使わない書き方。各 char を長さ 1 の String に起こせば slice::join が使える。
//      依存を足せない環境向けだが、1 文字ごとに String を確保するので本来は無駄が多い
// let ans = s.iter().map(|c| c.to_string()).collect::<Vec<_>>().join("o");

// alt: 空文字列をパターンにした replace は「全ての文字境界」にマッチするので、
//      "mtr".replace("", "o") は先頭と末尾にも付いて "omotoro" になる。両端 1 バイトを
//      削れば答え。短いが、S が英小文字 (1 バイト) である前提と長さ 1 で壊れる前提の
//      2 つに寄りかかっており、この問題の制約 (長さ 2 以上・英小文字のみ) でだけ安全
// input! { s: String };
// let t = s.replace("", "o");
// println!("{}", &t[1..t.len() - 1]);

// alt: 手で組み立てる版。先に 1 文字目を入れておき、2 文字目以降を「o と一緒に」押し込むと
//      「先頭かどうか」の分岐が skip(1) に吸収されて if が消える
// let mut ans = String::from(s[0]);
// for &c in s.iter().skip(1) {
//     ans.push('o');
//     ans.push(c);
// }
