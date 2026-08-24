use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    };

    let mut ans: Vec<usize> = vec![];
    let mut users = HashSet::new();

    for i in 0..n {
        if !users.contains(&s[i]) {
            users.insert(&s[i]);
            ans.push(i + 1);
        }
    } 

    for a in ans {
        println!("{}", a);
    }
}
