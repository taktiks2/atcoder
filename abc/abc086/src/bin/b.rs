use proconio::input;

fn main() {
    input! {
        mut a: String,
        b: String,
    };

    a.push_str(&b);

    let n: i32 = a.parse().unwrap();

    let mut ans = "No";
    let mut i = 1;
    loop {
        if i * i > n {
            break;
        } else if i * i == n {
            ans = "Yes";
            break;
        }
        i += 1;
    }

    println!("{}", ans)
}
