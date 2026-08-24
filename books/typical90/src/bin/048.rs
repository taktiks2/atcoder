use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    };

    let mut nums: Vec<usize> = vec![];
    for _ in 0..n {
        input! {
            a: usize,
            b: usize,
        };

        nums.push(b);
        nums.push(a - b);
    }

    nums.sort_by(|a, b| b.cmp(a));
    println!("{:?}", nums.iter().take(k).sum::<usize>())
}
