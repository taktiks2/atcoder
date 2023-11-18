use proconio::input;

fn main() {
    input! {
        _: usize,
        q: usize,
        s: String,
        lr: [(i32, i32); q],
    };

    for (l, r) in lr {
        let ss: &str = &s;
        let part = &ss[((l - 1) as usize)..(r as usize)];

        let mut cnt = 0;
        let mut prev_c: char = ' ';
        for c in part.chars() {
            if c == prev_c {
                cnt += 1;
            }
            prev_c = c;
        }
        println!("{}", cnt);
    }
}
