use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
        lr: [(usize, usize); q]
    };

    let mut hit: Vec<usize> = vec![];
    let mut miss: Vec<usize> = vec![];
    hit.push(0);
    miss.push(0);
    for i in 1..=n {
        hit.push(hit[i - 1]);
        if a[i - 1] == 1 {
            hit[i] += 1;
        }
        miss.push(miss[i - 1]);
        if a[i - 1] == 0 {
            miss[i] += 1;
        }
    }

    for (l, r) in lr {
        let hit_count = hit[r] - hit[l - 1];
        let miss_count = miss[r] - miss[l - 1];

        match hit_count.cmp(&miss_count) {
            std::cmp::Ordering::Equal => println!("draw"),
            std::cmp::Ordering::Greater => println!("win"),
            std::cmp::Ordering::Less => println!("lose"),
        }
    }

    // let acc_list = a
    //     .iter()
    //     .scan(0, |acc, &x| {
    //         *acc += x;
    //         Some(*acc)
    //     })
    //     .collect::<Vec<usize>>();
    //
    // for (l, r) in lr {
    //     let acc_part = acc_list[r - 1] - if l == 1 { 0 } else { acc_list[l - 2] };
    //     let all = r - l + 1;
    //     let half = all / 2;
    //
    //     if all % 2 == 0 {
    //         if acc_part > half {
    //             println!("win");
    //         } else if acc_part == half {
    //             println!("draw");
    //         } else {
    //             println!("lose");
    //         }
    //     } else {
    //         if acc_part >= half + 1 {
    //             println!("win");
    //         } else {
    //             println!("lose");
    //         }
    //     }
    // }
}
