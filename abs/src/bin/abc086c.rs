use proconio::input;

#[derive(Debug)]
struct Pos {
    t: i32,
    x: i32,
    y: i32,
}

fn main() {
    input! {
        n: i32,
    };

    let mut ans = "Yes";

    let mut pos = Pos { t: 0, x: 0, y: 0 };

    for _ in 0..n {
        input! {
            t: i32,
            x: i32,
            y: i32,
        };

        let dist = (x - pos.x).abs() + (y - pos.y).abs();
        let diff_time = t - pos.t;

        if diff_time < dist {
            ans = "No";
            break;
        }

        if (diff_time % 2 == 0 && dist % 2 == 0) || (diff_time % 2 == 1 && dist % 2 == 1) {
            pos = Pos { t, x, y };
        } else {
            ans = "No";
            break;
        }
    }

    println!("{}", ans);
}
