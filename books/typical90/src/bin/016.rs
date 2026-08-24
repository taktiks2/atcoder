use proconio::input;

// 全探索

fn main() {
    input! {
        n: i64,
        a: i64,
        b: i64,
        c: i64,
    };

    let mut ans: i64 = 10000;

    let l = 9999;

    for i in 0..=l {
        for j in 0..=l - i {
            let remain = n - (a * i + b * j);
            if remain >= 0 && remain % c == 0 {
                ans = ans.min(i + j + (remain / c));
            }
        }
    }

    println!("{}", ans);
}
