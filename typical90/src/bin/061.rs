use proconio::input;

fn main() {
    input! {
        q: usize,
        tx: [(i32, i32); q],
    };

    let mut deck: Vec<i32> = vec![];

    for (t, x) in tx {
        match t {
            1 => deck.insert(0, x),
            2 => deck.push(x),
            3 => println!("{}", deck[(x - 1) as usize]),
            _ => (),
        }
    }
}
