use proconio::input;

fn main() {
    input! {
        mut n: u64,
    };

    loop {
        if n % 2 == 0 {
            n /= 2;
        } else {
            break;
        }
    }

    loop {
        if n % 3 == 0 {
            n /= 3;
        } else {
            break;
        }
    }

    if n == 1 {
        println!("Yes");
    } else {
        println!("No");
    }
}
