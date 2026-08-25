# proconio 入力チートシート

テンプレートの `proconio = "=0.5.0"`（`features = ["derive"]`）で全例のコンパイルを確認済み。

## 基本形

```rust
use proconio::input;

input! {
    n: usize,
    a: i64,
    b: f64,
}
```

- 空白・改行は区別されないので、入力フォーマットの改行位置は気にしなくてよい
- 宣言した変数は immutable。後で書き換えるなら `mut a: i64` と書く

## 型指定一覧

| 書き方 | 読み取る内容 |
|---|---|
| `usize`, `i64`, `f64`, ... | `FromStr` を実装する任意の型 |
| `String` | 空白区切りの 1 トークン |
| `Chars` | 1 トークンを `Vec<char>` に |
| `Bytes` | 1 トークンを `Vec<u8>` に |
| `Usize1` | 1-indexed 入力を読んで **-1** した `usize` |
| `Isize1` | 同上の `isize` 版 |
| `(A, B, ...)` | タプル |
| `[T; n]` | 長さ `n` の `Vec<T>`（`n` は同じ `input!` 内で先に読んだ変数を使える） |
| `[T]` | **長さ自体を入力から読む** `Vec<T>`（先頭に個数が付く形式） |

マーカー型は `use proconio::marker::{Chars, Bytes, Usize1, Isize1};` でインポートする。

## 頻出パターン

### N と数列

```text
N
A_1 A_2 ... A_N
```

```rust
input! {
    n: usize,
    a: [i64; n],
}
```

### 1-indexed の添字（頂点番号など）

```rust
input! {
    n: usize,
    m: usize,
    edges: [(Usize1, Usize1); m],       // "1 2" → (0, 1)
    weighted: [(Usize1, Usize1, i64); m], // 重み付き辺
}
```

### 2 次元配列（数値グリッド）

```text
N M
A_11 ... A_1M
...
A_N1 ... A_NM
```

```rust
input! {
    n: usize,
    m: usize,
    grid: [[i64; m]; n],
}
```

### 文字盤面

```text
H W
S_1
...
S_H
```

```rust
input! {
    h: usize,
    w: usize,
    board: [Chars; h], // board[i][j] で 1 文字ずつアクセス
}
```

### 行ごとに長さが違う入力（先頭に個数付き）

```text
3
2 10 20
3 1 2 3
1 5
```

```rust
input! {
    n: usize,
    rows: [[i64]; n], // 各行の先頭の個数を長さとして読む
}
```

### クエリを逐次読む（形式がクエリ種別で変わる）

`input!` は複数回呼べる。続きから読まれる。

```rust
input! { q: usize }
for _ in 0..q {
    input! { kind: u32 }
    match kind {
        1 => { input! { pos: Usize1, val: i64 } /* 更新 */ }
        _ => { input! { l: Usize1, r: usize }   /* 区間取得 */ }
    }
}
```

### ソートなどで書き換える場合

```rust
input! {
    n: usize,
    mut a: [i64; n],
}
a.sort();
```

## 出力高速化: `#[fastout]`

出力が多い問題（10^5 行以上の `println!` など）では必須。

```rust
use proconio::fastout;

#[fastout]
fn main() {
    // println! がバッファリングされ、main 終了時に一括 flush される
}
```

⚠️ **インタラクティブ問題では `#[fastout]` を使わない**こと（flush されず応答が返らない）。

## インタラクティブ問題

`input_interactive!` を使う（行単位で読み、都度 flush される）。

```rust
use proconio::input_interactive;

println!("? {}", query);          // 質問を出力（fastout は付けない）
input_interactive! {
    reply: i64,                   // ジャッジの応答を読む
}
```

## ソースを明示する（`from`）

テストや自前バッファから読みたいとき。

```rust
use proconio::source::once::OnceSource;

let src = OnceSource::from("3\n1 2 3\n");
input! {
    from src,
    n: usize,
    a: [i64; n],
}
```

インタラクティブを `LineSource` で書く旧スタイル（`input_interactive!` と等価）:

```rust
use proconio::source::line::LineSource;

let stdin = std::io::stdin();
let mut src = LineSource::new(stdin.lock());
input! {
    from &mut src, // 使い回すときは &mut で渡す
    reply: i64,
}
```

## 参考

- ドキュメント: <https://docs.rs/proconio/0.5.0/proconio/>
