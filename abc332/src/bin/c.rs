use proconio::input;

fn main() {
    input! {
        _: usize,
        m: usize,
        s: String,
    };

    let mut continue_group: Vec<Vec<usize>> = Vec::new();
    let mut group: Vec<usize> = Vec::new();
    for c in s.chars() {
        match c {
            '1' | '2' => group.push(c.to_digit(10).unwrap() as usize),
            _ => {
                if !group.is_empty() {
                    continue_group.push(group);
                    group = Vec::new();
                }
            }
        }
    }
    if !group.is_empty() {
        continue_group.push(group);
    }

    let mut t2 = 0;
    let mut len = 0;
    for g in continue_group {
        let mut temp_t2 = 0;
        for &el in &g {
            if el == 2 {
                temp_t2 += 1;
            }
        }
        t2 = t2.max(temp_t2);
        len = len.max(g.len());
    }

    if m + t2 < len {
        t2 += len - (m + t2);
    }

    println!("{}", t2)
}
