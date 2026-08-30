// algo: brute-force
use proconio::input;

// bit全探索
//
// 各文字が「(」「)」の 2 択 × n 文字なので候補は 2^n 通り。n <= 20 なら 100 万通りで回りきる。
// bit の各ビットを 1 文字に対応させ (0 = '(' 、1 = ')')、全候補を作って正しい括弧列だけ残す。
// '(' < ')' なので「0 = '('」かつ「上位ビット = 左端の文字」にすると bit の昇順 = 辞書順になる。

fn main() {
    input! {
        n: usize,
    };

    // 答えを 1 行ずつ溜めるバッファ。println! を 1 万回以上叩くより最後に 1 回出す方が速い
    let mut out = String::new();

    // 0 から 2^n - 1 まで、あり得る文字列を昇順に全部試す (この順番がそのまま辞書順になる)
    for bit in 0..1usize << n {
        // 左から見たときの「開いたまま閉じていない括弧の数」。'(' で +1、')' で -1 する
        let mut depth = 0i32;
        // 途中で閉じすぎたかどうかのフラグ。false になったらこの bit は不正
        let mut ok = true;

        // i は「下から何番目のビットか」。rev() で i = n-1 → 0 と回すことで、
        // 上位ビット = 左端の文字となり、文字列を左から右へ走査する形になる
        for i in (0..n).rev() {
            // i 番目のビットを取り出して、0 なら '(' として +1、1 なら ')' として -1
            depth += if bit >> i & 1 == 0 { 1 } else { -1 };

            // depth が負 = まだ開いていない括弧を閉じた = この時点で不正確定なので打ち切る
            if depth < 0 {
                ok = false;
                break;
            }
        }

        // 「途中で閉じすぎない」かつ「最後に開きっぱなしが残らない」なら正しい括弧列
        if ok && depth == 0 {
            // ここで初めて文字列を組み立てる (不正な bit で無駄に作らないよう判定と分けている)
            for i in (0..n).rev() {
                out.push(if bit >> i & 1 == 0 { '(' } else { ')' });
            }
            out.push('\n');
        }
    }

    // 溜めた結果をまとめて出力。out は末尾に改行を含むので print! を使う
    print!("{out}");
}

// alt: 枝刈り DFS。「開きすぎない」「閉じすぎない」条件だけで正しい括弧列しか作らないので、
//      探索数が 2^n からカタラン数 (n=20 で 16796) に落ちる。判定コードも不要になる
// fn dfs(n: usize, open: usize, close: usize, cur: &mut String, out: &mut String) {
//     if cur.len() == n {
//         out.push_str(cur);
//         out.push('\n');
//         return;
//     }
//     if open < n / 2 {
//         cur.push('(');
//         dfs(n, open + 1, close, cur, out);
//         cur.pop();
//     }
//     if close < open {
//         cur.push(')');
//         dfs(n, open, close + 1, cur, out);
//         cur.pop();
//     }
// }
