# atcoder

AtCoder の解答リポジトリ（Rust）。[cargo-compete](https://github.com/qryxip/cargo-compete) + [just](https://github.com/casey/just) + Nix flake で運用する。

## セットアップ

Rust toolchain（rustc 1.89.0 = ジャッジと同一）と cargo-compete は flake が提供する。

```sh
direnv allow   # 初回のみ。以後は cd するだけで環境が整う
just login     # AtCoder にログイン（初回のみ）
```

## 使い方

```sh
just new abc338        # 新規コンテスト作成 + サンプル取得
# abc/abc338/src/bin/a.rs を編集
just test abc338 a     # サンプルでテスト (alias: just t)
just submit abc338 a   # テスト通過時のみ提出 (alias: just s)
```

| コマンド | 内容 |
| --- | --- |
| `just new <contest>` | パッケージ生成 + サンプル取得（生成先は種別ごとに自動振り分け） |
| `just test <contest> <problem>` | サンプルでテスト。`--release` などのフラグ追加可 |
| `just submit <contest> <problem>` | テスト通過時のみ提出し、ジャッジ結果を watch。`--no-test` / `--no-watch` 可 |
| `just open <contest>` | 問題ページとソース/テストを開く |
| `just retrieve <contest>` | テストケース再取得。`--full` でシステムテスト取得（Dropbox トークン要） |
| `just snippets` | 模範解答（`library/`）のテスト実行 + エディタ用スニペット再生成 |
| `just login` | AtCoder にログイン |

パッケージディレクトリ内なら `cargo compete test a` のように直接実行してもよい。

## ディレクトリ構成

```
abc/      AtCoder Beginner Contest
arc/ agc/ ahc/   （初回作成時に自動生成）
books/    書籍・問題集（tessoku-book, typical90）
other/    その他（abs など）
library/  アルゴリズムの模範解答（テスト付き、スニペットの元）
snippets/ library から生成した VSCode 形式スニペット（生成物、コミット対象）
```

- 1 コンテスト = 1 Cargo パッケージ、1 問題 = 1 bin ターゲット（`src/bin/<problem>.rs`）
- 問題とバイナリの対応は各パッケージの `Cargo.toml` の `[package.metadata.cargo-compete.bin]`
- テストケースは `<contest>/testcases/<problem>.yml`。`cases:` に入出力を追記すれば自作ケースを足せる

## スニペット（模範解答ライブラリ）

アルゴリズムの典型コードを [cargo-snippet](https://github.com/hatoo/cargo-snippet) で管理する。
`library/` に `#[snippet]` 付き関数 + テストとして書き、`snippets/rust.json`（VSCode 形式）に変換して
Neovim（blink.cmp）から展開する。展開後のコードは自己完結しているのでそのまま提出できる。

### スニペットの追加手順

1. `library/src/` にモジュールを作り、`#[snippet]`（または `#[snippet("名前")]`）付き関数とテストを書く
   - 展開後に自己完結するよう、`use` は関数内に書く
   - `library/src/lib.rs` に `pub mod 名前;` を追加
2. `just snippets` を実行（テストが通ったときだけ `snippets/rust.json` が再生成される）
3. Neovim を再起動（blink.cmp は起動時にスニペットを読んでキャッシュするため）

### 収録スニペット

| prefix | 内容 |
| --- | --- |
| `binary_search` | めぐる式二分探索（`meguru_bisect`） |
| `bfs` | 幅優先探索（隣接リスト、始点からの距離） |
| `dfs` | 深さ優先探索（再帰、行きがけ順） |

### Neovim 側の設定

blink.cmp の snippets ソースの `search_paths` に本リポジトリの `snippets/` を追加してある
（dotfiles の `.config/nvim/lua/plugins/coding.lua`）。nvim 設定は home-manager 管理のため、
設定変更時は dotfiles 側で `darwin-rebuild switch` が必要。スニペットの**再生成だけなら rebuild 不要**。

## ジャッジ環境（2025/10 言語アップデート準拠）

- Rust **1.89.0** / edition **2024** / language_id **6088**（2025-10-18 の ABC428 以降）
- 利用可能クレート: [言語・ライブラリ一覧](https://img.atcoder.jp/file/language-update/2025-10/language-list.html) / [インストール仕様](https://img.atcoder.jp/file/language-update/2025-10/088-1-82-0_rustc.toml)
- `compete.toml` のテンプレート依存クレートと `template-cargo-lock.toml` は、ジャッジの構成（[rust-lang-ja/atcoder-proposal](https://github.com/rust-lang-ja/atcoder-proposal)）と同一バージョンに固定している
- ジャッジが更新されたら: flake.nix の rustc バージョン、compete.toml の `language_id`・`edition`・依存クレート、template-cargo-lock.toml を追従させる

## 補足

- 2025/10 より前のパッケージ（abc063〜abc337 など）は旧ジャッジ時代の依存のまま。提出設定はソースファイル単体送信なので提出には影響しない
- 認証情報は `~/Library/Application Support/cargo-compete/` に保存される（コミット対象外）
