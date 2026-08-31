// algo: binary-search
use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        q: usize,
        b: [i64; q],
    };

    let mut class = a.clone();
    class.sort_unstable();

    for bi in b {
        let mut left = 0;
        let mut right = n - 1;

        let mut ans = 0i64;

        if bi <= class[left] {
            ans = class[left] - bi;
        } else if bi >= class[right] {
            ans = bi - class[right];
        } else {
            while right - left > 1 {
                let mid = (left + right) / 2;
                let sub_1 = class[mid] - bi;
                let sub_2 = class[mid + 1] - bi;

                if sub_1 <= 0 && sub_2 >= 0 {
                    left = mid;
                    right = mid + 1;
                }

                if sub_1 > 0 {
                    right = mid;
                }

                if sub_2 < 0 {
                    left = mid + 1;
                }
            }

            ans = min((class[left] - bi).abs(), (class[right] - bi).abs());
        }

        println!("{ans}");
    }
}

// alt: 標準ライブラリの partition_point (C++ の lower_bound 相当) を使うと手書き二分探索が丸ごと消える。
//      返り値 i は「bi 以上の値が現れる最初の位置」なので、候補は class[i] (bi 以上で最小) と
//      class[i-1] (bi 未満で最大) の 2 つだけ。i == 0 / i == n の端のケースも同じ 2 つの if に
//      吸収されるため、ループ前の class[left]/class[right] との場合分けも不要になる
// for bi in b {
//     let i = class.partition_point(|&x| x < bi);
//     let mut ans = i64::MAX;
//     if i < n {
//         ans = ans.min(class[i] - bi);
//     }
//     if i > 0 {
//         ans = ans.min(bi - class[i - 1]);
//     }
//     println!("{ans}");
// }

// alt: else 節の手書き二分探索は 3 条件に分岐しているが、「class[left] <= bi < class[right]」を
//      不変条件にすると 001 と同じ ok/ng 形式の 1 比較に潰せる。事前の場合分けで
//      class[0] < bi < class[n-1] が保証されているので、初期値 left = 0, right = n - 1 で
//      不変条件が成立する。終了時は左右とも符号が確定しているので abs() も要らない
// while right - left > 1 {
//     let mid = (left + right) / 2;
//     if class[mid] <= bi {
//         left = mid;
//     } else {
//         right = mid;
//     }
// }
// ans = min(bi - class[left], class[right] - bi);

// alt: input! で mut を付ければ a をそのままソートでき、clone (最大 3*10^5 要素のコピー) が消える。
//      元の並び順はどこでも使っていないので複製を残す理由がない
// mut a: [i64; n],
// a.sort_unstable();

// alt: Q <= 3*10^5 行を println! で 1 行ずつ出すと毎回 stdout のロック取得が走る。
//      #[proconio::fastout] を main に付けるだけで出力が BufWriter に溜まり、終了時に 1 回で流れる
// #[proconio::fastout]
// fn main() {
