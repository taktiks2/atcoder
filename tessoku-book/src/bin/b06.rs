use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
        lr: [(usize, usize); q]
    };

    let acc_list = a
        .iter()
        .scan(0, |acc, &x| {
            *acc += x;
            Some(*acc)
        })
        .collect::<Vec<usize>>();

    for (l, r) in lr {
        let acc_part = acc_list[r - 1] - if l == 1 { 0 } else { acc_list[l - 2] };
        let all = r - l + 1;
        let half = all / 2;

        if all % 2 == 0 {
            if acc_part > half {
                println!("win");
            } else if acc_part == half {
                println!("draw");
            } else {
                println!("lose");
            }
        } else {
            if acc_part >= half + 1 {
                println!("win");
            } else {
                println!("lose");
            }
        }
    }
}
