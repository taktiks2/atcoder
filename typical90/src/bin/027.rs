use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    };

    let mut ans: Vec<usize> = vec![];
    let mut users = String::from("");

    for i in 0..n {
        if !users.contains(&s[i]) {
            users.push_str(&format!("{},", s[i]));
            ans.push(i + 1);
        }
    } 

    for a in ans {
        println!("{}", a);
    }
}
