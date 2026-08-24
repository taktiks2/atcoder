use proconio::input;

fn main() {
    input! {
        n: u32,
    };

    let mut class_1: Vec<i32> = Vec::new();
    let mut class_2: Vec<i32> = Vec::new();

    let mut sum_1: i32 = 0;
    let mut sum_2: i32 = 0;

    for _ in 0..n {
        input! {
            c: i32,
            p: i32,
        };

        if c == 1 {
            sum_1 += p;
        } else {
            sum_2 += p;
        }

        class_1.push(sum_1);
        class_2.push(sum_2);
    }

    input! {
        q: usize,
    };

    for _ in 0..q {
        input! {
            l: usize,
            r: usize,
        };

        let sub_1 = if l < 2 { 0 } else { class_1[l - 2] };
        let sub_2 = if l < 2 { 0 } else { class_2[l - 2] };

        println!("{} {}", class_1[r - 1] - sub_1, class_2[r - 1] - sub_2,);
    }
}
