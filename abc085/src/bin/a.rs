use proconio::input;

fn main() {
    input! {
        s: String,
    };

    let ans = s.replace("2017", "2018");

    println!("{}", ans);
}
