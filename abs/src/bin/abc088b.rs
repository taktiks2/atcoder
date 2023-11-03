use proconio::input;

fn main() {
    input! {
        n: u32,
        mut a: [u32; n],
    };

    let mut alice = 0;
    let mut bob = 0;

    a.sort_by(|a, b| b.cmp(a)); // 降順にソート
    for (index, element) in a.iter().enumerate() {
        if index % 2  == 1 {
            bob += element;
        } else {
            alice += element;
        }
    }

    println!("{}", alice - bob);
}
