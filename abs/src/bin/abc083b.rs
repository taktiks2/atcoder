use proconio::input;

fn main() {
    input! {
        nab: [u32; 3],
    };

    let mut result = 0;

    for n in 1..=nab[0] {
        let mut ttl = 0;
        for nc in n.to_string().as_str().chars() {
            ttl += nc.to_digit(10).unwrap();
        }
        if ttl >= nab[1] && ttl <= nab[2] {
            result += n;
        }
    }

    println!("{}", result);
}
