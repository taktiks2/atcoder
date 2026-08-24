use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
    };

    if a + b >= 10 {
        println!("error");
    } else {
        println!("{}", a + b);
    }
}
