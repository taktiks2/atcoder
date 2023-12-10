use proconio::input;

fn main() {
    input! {
        k: usize,
        g: isize,
        m: isize,
    };

    let mut glass = 0;
    let mut mag = 0;
    for _ in 0..k {
        if glass == g {
            glass = 0;
        } else if mag == 0 {
            mag = m;
        } else {
            let remain = g - glass;
            if mag - remain < 0 {
                glass += mag;
                mag = 0;
            } else {
                glass += remain;
                mag -= remain;
            }
        }
    }
    println!("{} {}", glass, mag);
}
