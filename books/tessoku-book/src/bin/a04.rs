use proconio::input;

fn main() {
    input! {
        n: usize
    };

    for i in (0..=9).rev() {
        // 1 << i は 2^i になる
        print!("{}", n / (1 << i) % 2)
    }
}

// fn main() {
//     input! {
//         n: usize
//     };
//
//     let mut dividend = n;
//     let mut ans = vec![];
//     loop {
//         if dividend == 1 {
//             ans.push(1);
//             break;
//         }
//         if dividend % 2 == 0 {
//             ans.push(0);
//         } else {
//             ans.push(1)
//         }
//         dividend /= 2;
//     }
//
//     for i in (0..10).rev() {
//         if i < ans.len() {
//             print!("{}", ans[i]);
//         } else {
//             print!("0");
//         }
//     }
// }
