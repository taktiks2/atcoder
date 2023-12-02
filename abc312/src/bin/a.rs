use proconio::input;

fn main() {
    input! {
        s: String,
    };

    let list = ["ACE", "BDF", "CEG", "DFA", "EGB", "FAC", "GBD"];

    if list.iter().any(|&item| item == s) {
        println!("Yes");
    } else {
        println!("No");
    }
}
