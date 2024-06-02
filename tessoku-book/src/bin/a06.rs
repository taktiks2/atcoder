use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
    };

    let mut acc_list = vec![];
    let mut acc = 0;
    for ai in a {
        acc += ai;
        acc_list.push(acc);
    }

    for _ in 0..q {
        input! {
            l: usize,
            r: usize
        };

        if l == 1 {
            println!("{}", acc_list[r - 1]);
            continue;
        }
        println!("{}", acc_list[r - 1] - acc_list[l - 2])
    }
}
