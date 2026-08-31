//! アルゴリズムの模範解答（スニペット）集。
//!
//! 各関数は `#[snippet]` 付きで、`just snippets` で VSCode 形式の
//! スニペット (snippets/rust.json) に変換され Neovim (blink.cmp) から展開できる。
//! 展開後のコードが自己完結するよう、use は関数内に書く。

pub mod bfs;
pub mod binary_search;
pub mod dfs;
