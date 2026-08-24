use proconio::input;

fn main() {
    input! {
        n: i32,
    };

    for i in n..=919 {
        let i = i.to_string();
        let hundleds = i.chars().next().unwrap().to_digit(10).unwrap();
        let tens = i.chars().nth(1).unwrap().to_digit(10).unwrap();
        let one = i.chars().nth(2).unwrap().to_digit(10).unwrap();

        if hundleds * tens == one {
            println!("{}", i);
            break;
        }
    }
}
