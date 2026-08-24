use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    };

    let a_clone = a.clone();

    let unique: HashSet<_> = a.into_iter().collect();
    let mut unique_vec: Vec<_> = unique.into_iter().collect();

    unique_vec.sort();

    let mut b = vec![0; n];

    for (i, e) in a_clone.iter().enumerate() {
        let target = unique_vec.binary_search(e);
        match target {
            Ok(t) => b[i] = t + 1,
            Err(_) => continue,
        }
    }

    let b_str: Vec<String> = b.iter().map(|x| x.to_string()).collect();

    let ans = b_str.join(" ");
    println!("{}", ans)
}
