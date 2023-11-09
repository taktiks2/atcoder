use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    };

    let mut ans: Vec<usize> = vec![];
    let mut users: Vec<String> = vec![];

    for i in 0..n {
        if users.iter().any(|user| user == &s[i]) {
            continue;
        } else {
            ans.push(i + 1);
            users.push(s[i].clone());
        }
    }

    for a in ans {
        println!("{}", a);
    }
}
