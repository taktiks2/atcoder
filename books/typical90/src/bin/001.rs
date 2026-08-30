// algo: binary-search
use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        k: usize,
        a: [usize; n],
    };

    let can = |x: usize| -> bool {
        let mut prev = 0;
        let mut pieces = 0;
        for &ai in &a {
            if ai - prev >= x {
                pieces += 1;
                prev = ai;
            }
        }
        if l - prev >= x {
            pieces += 1;
        }
        pieces > k
    };

    let mut ok = 0;
    let mut ng = l + 1;

    while ng - ok > 1 {
        let mid = (ng + ok) / 2;
        if can(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{ok}");
}

// alt: 上限は「均等に割った長さ」を超えないので ng = l / (k + 1) + 1 まで狭められる。
//      判定回数が 30 回 → 25 回程度に減るだけで計算量は変わらないが、探索範囲の意味は明確になる
// let mut ng = l / (k + 1) + 1;

// alt: 判定を fold で書くと可変変数が消える。(切った回数, 直前の切り位置) を畳み込む
// let can = |x: usize| -> bool {
//     let (pieces, prev) = a.iter().fold((0usize, 0usize), |(cnt, prev), &ai| {
//         if ai - prev >= x { (cnt + 1, ai) } else { (cnt, prev) }
//     });
//     pieces + usize::from(l - prev >= x) >= k + 1
// };
