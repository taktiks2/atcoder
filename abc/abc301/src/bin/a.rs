use proconio::input;
use std::cmp::Ordering;

fn main() {
    input! {
        n: usize,
        s: String,
    };

    let draw_win = n / 2;

    let mut t_win = 0;
    let mut a_win = 0;
    let mut draw = 'B';
    s.chars().for_each(|c| {
        if c == 'A' {
            a_win += 1;
            if draw == 'B' && a_win == draw_win {
                draw = 'A';
            }
        } else {
            t_win += 1;
            if draw == 'B' && t_win == draw_win {
                draw = 'T';
            }
        }
    });

    let winner = match (t_win.cmp(&a_win), draw) {
        (Ordering::Greater, _) => 'T',
        (Ordering::Less, _) => 'A',
        (Ordering::Equal, 'A') => 'A',
        (Ordering::Equal, 'T') => 'T',
        _ => ' ',
    };

    println!("{}", winner)
}
