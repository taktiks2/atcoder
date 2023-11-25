use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        s: String,
    };

    let mut player_scores = HashMap::new();
    player_scores.insert("tourist", 3858);
    player_scores.insert("ksun48", 3679);
    player_scores.insert("Benq", 3658);
    player_scores.insert("Um_nik", 3648);
    player_scores.insert("apiad", 3638);
    player_scores.insert("Stonefeang", 3630);
    player_scores.insert("ecnerwala", 3613);
    player_scores.insert("mnbvmar", 3555);
    player_scores.insert("newbiedmy", 3516);
    player_scores.insert("semiexp", 3481);

    println!("{}", player_scores.get(s.as_str()).unwrap())
}
