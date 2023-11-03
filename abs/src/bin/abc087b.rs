use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
        x: i32,
    };

    let mut cnt = 0;

    for ai in 0..=a {
        for bi in 0..=b {
            let remain_amount = x - (ai * 500 + bi * 100);
            if remain_amount == 0 {
                cnt += 1;
            } else if remain_amount < 50 {
                break;
            } else if remain_amount % 50 == 0 && remain_amount / 50 <= c {
                cnt += 1;
            }
        }
    }

    println!("{}", cnt)
}
