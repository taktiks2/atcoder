use proconio::input;

fn convert_octal_to_nonary(num: &str) -> Vec<String> {
    let mut decimal = u64::from_str_radix(num, 8).unwrap();
    let mut nonary = vec![];

    while decimal >= 9 {
        nonary.push((decimal % 9).to_string());
        decimal /= 9;
    }

    if decimal != 0 {
        nonary.push(decimal.to_string());
    }

    nonary.reverse();
    nonary
}

fn convert_8_to_5(num: Vec<String>) -> String {
    let mut temp_num = num.clone();
    for (i, c) in num.iter().enumerate() {
        if c == "8" {
            temp_num[i] = "5".to_string();
        }
    }
    temp_num.join("")
}

fn main() {
    input! {
        n: String,
        k: usize,
    };

    if n == "0" {
        println!("0");
        return;
    }

    let mut ans = n.clone();
    for _ in 0..k {
        let nonary = convert_octal_to_nonary(&ans);
        ans = convert_8_to_5(nonary);
    }

    println!("{}", ans);
}
