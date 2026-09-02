// algo: prefix-sum
use proconio::{input, marker::Usize1};

// クラス別の累積和を 2 本持つ
//
// クエリごとに L..=R を舐めると O(NQ) = 10^10 で間に合わない。「クラス 1 だけの累積和」と
// 「クラス 2 だけの累積和」を先に作っておけば、1 クエリは引き算 2 回で済み、全体 O(N + Q)。
//
// C_i は 1/2 なので Usize1 で読んで 0/1 に落とし、そのまま配列の添字に使っている。
// これで if c == 1 { .. } else { .. } の分岐が消え、「自分のクラスに足す」がそのまま
// prefix_sum[i][c] += p という 1 行になる。
//
// 2 本を Vec<usize> 2 本ではなく Vec<[usize; 2]> にしているのは、1 クエリで読む 2 値が
// 隣り合っていて同じキャッシュラインに乗るため。

fn main() {
    input! {
        n: usize,
        students: [(Usize1, usize); n],
        q: usize,
        queries: [(Usize1, Usize1); q],
    };

    let mut prefix_sum = vec![[0usize, 0usize]; n];

    for (i, &(c, p)) in students.iter().enumerate() {
        if i > 0 {
            let hanten = c ^ 1;
            prefix_sum[i][c] += prefix_sum[i - 1][c] + p;
            prefix_sum[i][hanten] = prefix_sum[i - 1][hanten];
        } else {
            prefix_sum[i][c] = p;
            prefix_sum[i][c ^ 1] = 0;
        }
    }

    for (l, r) in queries {
        let a = prefix_sum[r][0] - if l > 0 { prefix_sum[l - 1][0] } else { 0 };
        let b = prefix_sum[r][1] - if l > 0 { prefix_sum[l - 1][1] } else { 0 };
        println!("{a} {b}");
    }
}

// alt: 先頭に番兵 (全部 0 の行) を 1 つ足して長さ n + 1 にすると、main にある 2 つの
//      「先頭だけ特別扱い」が両方消える。i == 0 の else 節も、クエリ側の if l > 0 も要らない。
//      さらに [usize; 2] は Copy なので「直前の行をまるごとコピーしてから自分のクラスに足す」と
//      書け、反転クラスを c ^ 1 で持ち回る必要もなくなる。
//      R は Usize1 を外して 1-origin のまま読むと、クエリが半開区間 [l, r) の差になり
//      +1 の調整も消える (L は Usize1 のまま = l、R はそのまま = r)
// queries: [(Usize1, usize); q],
//
// let mut prefix_sum = vec![[0usize; 2]; n + 1];
// for (i, &(c, p)) in students.iter().enumerate() {
//     prefix_sum[i + 1] = prefix_sum[i];
//     prefix_sum[i + 1][c] += p;
// }
// for (l, r) in queries {
//     let a = prefix_sum[r][0] - prefix_sum[l][0];
//     let b = prefix_sum[r][1] - prefix_sum[l][1];
//     println!("{a} {b}");
// }

// alt: 累積和は「途中経過を全部残す畳み込み」なので scan がそのまま当てはまる。
//      once([0; 2]) を chain すれば番兵込みで一度に組め、mut も添字への代入も消える。
//      scan のクロージャが受け取る acc がまさに「ここまでの合計」で、Some(*acc) で
//      各ステップのスナップショットを吐く形
// let prefix_sum: Vec<[usize; 2]> = std::iter::once([0usize; 2])
//     .chain(students.iter().scan([0usize; 2], |acc, &(c, p)| {
//         acc[c] += p;
//         Some(*acc)
//     }))
//     .collect();
