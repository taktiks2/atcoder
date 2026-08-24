use proconio::input;

fn main() {
    input! {
        m1: i32,
        d1: i32,
        mut y: i32,
        mut m2: i32,
        mut d2: i32,
    };

    if d1 == d2 {
        if m1 == m2 {
            y += 1;
            m2 = 1;
        } else {
            m2 += 1;
        }
        d2 = 1;
    } else {
        d2 += 1;
    }

    println!("{} {} {}", y, m2, d2);
}
