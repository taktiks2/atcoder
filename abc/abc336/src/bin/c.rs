use proconio::input;

fn base_5(mut x: usize) -> Vec<usize> {
    let mut v = Vec::new();
    if x == 0 {
        return vec![0];
    }
    while x > 0 {
        v.push(x % 5);
        x /= 5;
    }
    v
}

fn main() {
    input! {
        n: usize,
    };

    let mut arr = base_5(n - 1);
    arr.reverse();

    for i in arr {
        print!("{}", i * 2)
    }
}
