use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    };

    let mut ans = "No";

    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                if a[i] + a[j] + a[k] == 1000 {
                    ans = "Yes";
                }
            }
        }
    }

    println!("{}", ans);
}

// NOTE: 自分の回答

// fn main() {
//     input! {
//         n: usize,
//         a: [usize; n]
//     };
//
//     let mut ans = "No";
//
//     for i in &a {
//         for j in &a {
//             for k in &a {
//                 if i == j || j == k || i == k {
//                     continue;
//                 }
//                 if i + j + k == 1000 {
//                     ans = "Yes";
//                 }
//             }
//         }
//     }
//
//     println!("{}", ans);
// }
