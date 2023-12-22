use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let evens: Vec<usize> = a.into_iter().filter(|ai| ai % 2 == 0).collect();

    let evens_str: Vec<String> = evens.iter().map(|ai| ai.to_string()).collect();

    println!("{}", evens_str.join(" "))
}
