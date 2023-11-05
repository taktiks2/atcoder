use proconio::input;

fn main() {
    input! {
        a: [[u32; 9]; 9],
    };

    let mut cols = vec![];

    for wi in 0..9 {
        let mut temp_row = vec![0; 9];
        for row in a.iter() {
            temp_row.push(row[wi]);
        }
        cols.push(temp_row);
    }
}
