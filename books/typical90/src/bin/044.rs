use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
    };

    let mut shift = 0;
    for _ in 0..q {
        input! {
            t: usize,
            x: usize,
            y: usize,
        };

        match t {
            1 => {
                let mut temp_x: i32 = x as i32 - shift as i32 - 1;
                if temp_x < 0 {
                    temp_x += n as i32;
                }
                let shifted_x = temp_x as usize % n;
                let mut temp_y: i32 = y as i32 - shift as i32 - 1;
                if temp_y < 0 {
                    temp_y += n as i32;
                }
                let shifted_y = temp_y as usize % n;
                a.swap(shifted_x, shifted_y);
            }
            2 => {
                shift = (shift + 1) % n;
            }
            3 => {
                let mut temp_x: i32 = x as i32 - shift as i32 - 1;
                if temp_x < 0 {
                    temp_x += n as i32;
                }
                let shifted_x = temp_x as usize % n;
                println!("{}", a[shifted_x])
            }
            _ => {}
        }
    }
}
