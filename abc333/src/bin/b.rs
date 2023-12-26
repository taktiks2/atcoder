use proconio::input;

const TABLE: [[usize; 5]; 5] = [
    [0, 1, 2, 2, 1],
    [1, 0, 1, 2, 2],
    [2, 1, 0, 1, 2],
    [2, 2, 1, 0, 1],
    [1, 2, 2, 1, 0],
];

fn convert_letter_to_number(input: &str) -> Vec<usize> {
    input
        .chars()
        .filter_map(|c| match c {
            'A' => Some(0),
            'B' => Some(1),
            'C' => Some(2),
            'D' => Some(3),
            'E' => Some(4),
            _ => None,
        })
        .collect()
}

fn main() {
    input! {
        s: String,
        t: String,
    };

    let s_num = convert_letter_to_number(&s);
    let t_num = convert_letter_to_number(&t);

    if TABLE[s_num[0]][s_num[1]] == TABLE[t_num[0]][t_num[1]] {
        println!("Yes")
    } else {
        println!("No")
    }
}
