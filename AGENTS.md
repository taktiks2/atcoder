# AGENTS.md

AtCoder の解答リポジトリ（Rust）。セットアップ・コマンド・ディレクトリ構成の詳細は [README.md](README.md) を参照。

## 前提

- ユーザーは解答コードを自分で写経して学ぶ。頼まれない限り `abc/` などの解答ファイルに解法を書き込まない（`annotate` スキルによるコメント付与は例外）
- ジャッジは Rust 1.89.0 / edition 2024。toolchain は flake が提供する（ジャッジと同一）

## 配置

- 1 コンテスト = 1 Cargo パッケージ、1 問題 = `src/bin/<problem>.rs`（例: `abc/3xx/abc338/src/bin/a.rs`）
- テストケースは `<contest>/testcases/<problem>.yml`
- `library/` はスニペット元の模範実装（テスト付き）、`snippets/rust.json` はその生成物
- `// algo:` / `// alt:` タグの語彙は `TAGS.md`。初出のタグは TAGS.md に追記してから使う

## コマンド

- `just test <contest> <problem>` — サンプルでテスト
- `just snippets` — `library/` のテスト + スニペット再生成
- `just algo <tag>` / `just algos` — タグ検索 / 集計
- 提出（`just submit`）はユーザーが行う

## Rust のコード規約

- 入力は proconio。出力は `#[fastout]` + `println!` で書く（テンプレートが `#[fastout]` 前提）。`BufWriter` を手書きしない
- 例外: 対話型（インタラクティブ）問題では `#[fastout]` を使わない
- `library/` の `#[snippet]` 関数は、展開後に自己完結するよう `use` を関数内に書く
