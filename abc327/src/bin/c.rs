use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        a: [[i32; 9]; 9],
    };

    for row in &a {
        let hash_row: HashSet<_> = row.iter().copied().collect();
        if hash_row.iter().sum::<i32>() != 45 {
            println!("No");
            return;
        }
    }

    for i in 0..9 {
        let mut hash_col = HashSet::new();
        for row in a.iter() {
            hash_col.insert(row[i]);
        }
        if hash_col.iter().sum::<i32>() != 45 {
            println!("No");
            return;
        }
    }

    for y in 0..3 {
        for x in 0..3 {
            let mut hash_sql = HashSet::new();
            for i in (y * 3)..(y * 3 + 3) {
                for j in (x * 3)..(x * 3 + 3) {
                    hash_sql.insert(a[i][j]);
                }
            }
            if hash_sql.iter().sum::<i32>() != 45 {
                println!("No");
                return;
            }
        }
    }

    println!("Yes");
}
