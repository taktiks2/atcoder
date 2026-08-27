// algo: sort, simulation
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };

    let (mut positives, mut negatives): (Vec<i64>, Vec<i64>) =
        a.into_iter().partition(|&x| x > 0i64);

    positives.sort();
    negatives.sort_by(|a, b| b.cmp(a));

    let mut pos = 0i64;
    let mut positive_i = 0;
    let mut negative_i = 0;
    let mut ans = 0i64;

    while positives.len() > positive_i || negatives.len() > negative_i {
        if positives.len() > positive_i && negatives.len() > negative_i {
            let abs_positive = (positives[positive_i] - pos).abs();
            let abs_negative = (negatives[negative_i] - pos).abs();

            if abs_positive < abs_negative {
                pos = positives[positive_i];
                positive_i += 1;
                ans += abs_positive;
            } else {
                pos = negatives[negative_i];
                negative_i += 1;
                ans += abs_negative;
            }
        } else if positives.len() > positive_i {
            let abs_positive = (positives[positive_i] - pos).abs();
            pos = positives[positive_i];
            positive_i += 1;
            ans += abs_positive;
        } else {
            let abs_negative = (negatives[negative_i] - pos).abs();
            pos = negatives[negative_i];
            negative_i += 1;
            ans += abs_negative;
        }
    }

    println!("{}", ans);
}

// alt: Peekable + match で「どちらへ行くか」と「移動する」を分離し、3 箇所の移動処理の重複を 1 箇所に（Rustらしさ・可読性）
// - インデックス管理と境界チェックが peek()/next() に置き換わり、場合分けは match の網羅性チェックが保証してくれる
// - フロンティアは常に負側 < cur < 正側 なので距離は cur - x / y - cur と書け、abs() が不要になる
// - 降順ソートは sort_by(|a, b| b.cmp(a)) より sort_unstable_by_key(Reverse) が定石。プリミティブは sort_unstable で十分
#[allow(dead_code)]
fn alt_peekable() {
    use std::cmp::Reverse;

    input! {
        n: usize,
        a: [i64; n],
    };

    let (mut positives, mut negatives): (Vec<i64>, Vec<i64>) =
        a.into_iter().partition(|&x| x > 0);
    positives.sort_unstable();
    negatives.sort_unstable_by_key(|&x| Reverse(x));

    let mut pos_it = positives.into_iter().peekable();
    let mut neg_it = negatives.into_iter().peekable();
    let mut cur = 0i64;
    let mut ans = 0i64;

    loop {
        let go_neg = match (neg_it.peek(), pos_it.peek()) {
            (Some(&x), Some(&y)) => cur - x <= y - cur,
            (Some(_), None) => true,
            (None, Some(_)) => false,
            (None, None) => break,
        };
        let target = if go_neg { neg_it.next() } else { pos_it.next() }.unwrap();
        ans += (target - cur).abs();
        cur = target;
    }

    println!("{ans}");
}
