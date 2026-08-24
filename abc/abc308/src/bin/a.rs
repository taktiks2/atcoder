use proconio::input;

fn main() {
    input! {
        s: [i32; 8],
    };

    let mut ans = "Yes";

    for i in 0..8 {
        if i == 0 {
            if s[i] < 100 || s[i] > 675 || s[i] % 25 != 0 {
                ans = "No";
                break;
            }
            continue;
        }

        if s[i] < 100 || s[i] > 675 || s[i] % 25 != 0 || s[i - 1] > s[i] {
            ans = "No";
            break;
        }
    }

    println!("{}", ans);
}
