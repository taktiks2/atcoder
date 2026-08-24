use proconio::input;

fn main() {
    input! {
        x: i32,
        y: i32,
    };

    let diff = x - y;

    if diff > 0 {
        if diff < 4 {
            println!("Yes");
        } else {
            println!("No");
        }
    } else {
        if diff > -3 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
