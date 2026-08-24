use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String,
        t: String,
    };

    let vec_s = s.chars().collect::<Vec<char>>();
    let vec_t = t.chars().collect::<Vec<char>>();
    let mut ans = "Yes";
    for i in 0..n {
        if (vec_s[i] == 'o' || vec_s[i] == '0') && (vec_t[i] == 'o' || vec_t[i] == '0')
            || (vec_s[i] == 'l' || vec_s[i] == '1') && (vec_t[i] == 'l' || vec_t[i] == '1')
            || vec_s[i] == vec_t[i]
        {
            continue;
        } else {
            ans = "No";
            break;
        }
    }
    println!("{}", ans)
}
