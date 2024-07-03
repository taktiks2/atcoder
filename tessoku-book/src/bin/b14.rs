use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n]
    };

    let (a1, a2) = a.split_at(n / 2);

    let mut p: Vec<usize> = vec![];
    for i in 0..(1 << a1.len()) {
        let mut sum = 0;
        for j in 0..a1.len() {
            let divider = 1 << j;
            if (i / divider) % 2 == 1 {
                sum += a1[j]
            }
        }
        p.push(sum)
    }

    let mut q: Vec<usize> = vec![];
    for i in 0..(1 << a2.len()) {
        let mut sum = 0;
        for j in 0..a2.len() {
            let divider = 1 << j;
            if (i / divider) % 2 == 1 {
                sum += a2[j]
            }
        }
        q.push(sum)
    }
    q.sort();

    for pe in p {
        if k < pe {
            continue;
        }
        let target = k - pe;
        let index = q.binary_search(&target);
        match index {
            Ok(_) => {
                println!("Yes");
                return;
            }
            Err(_) => continue,
        }
    }
    println!("No")
}
