# testcases yml チートシート

`testcases/<問題>.yml` は `just test` / `just submit`（中身は `cargo compete test` / `submit`）が読むテスト定義。
本文の挙動は cargo-compete 0.10.7 で実測して確認済み。

## コンテスト中によく使う操作

| やりたいこと | 方法 |
| --- | --- |
| サンプルを流す | `just test abc210 c` |
| 特定ケースだけ流す | `just test abc210 c --testcases sample2 big` |
| 自作ケースを足す | `cases:` に追記するか、`testcases/c/in/foo.txt` と `out/foo.txt` を置く |
| 出力を目視するだけ（正解が分からない） | `out:` を省略したケースを足す（比較されず、常に Accepted） |
| 誤差許容の問題 | `match:` を `Float` に書き換える |
| 答えが複数ある問題 | `match:` を `Checker` にして検証スクリプトを書く |
| TLE を検出したい | `timelimit: 2s` を書く（`~` は無制限） |
| テストが WA のまま提出したい | `just submit abc210 c --no-test` |

`just submit` は提出前にテストを走らせ、全部通ったときだけ提出する。`match` が問題の判定方法と合っていないと、正しいコードでも提出がブロックされる。

## 全体の構造

```yaml
---
type: Batch          # 通常の問題
timelimit: ~         # ~ は無制限。2s のように書くと超過で Timelimit Exceeded
match: Lines         # 出力の比較方法（下表）

cases:               # sample は問題ページから自動で入る
  - name: sample1
    in: |
      7 3
      1 2 1 2 3 3 1
    out: |           # 省略すると比較せず、実行するだけ
      3

extend:              # 外部ファイルからもケースを読む
  - type: Text
    path: "./c"      # testcases/c/ 以下
    in: /in/*.txt
    out: /out/*.txt  # in/foo.txt と out/foo.txt がファイル名で対応する
```

- `extend` のファイルは、ケース名がファイル名の拡張子なし部分になる（`in/big.txt` → `big`）
- `out/` に対応するファイルが無い入力は、`out:` 省略と同じ扱いになる
- `testcases/c/` ディレクトリは `just test` が自動で作る
- `timelimit` は `[test] profile` の既定（`dev` = デバッグビルド）で計測するので、ジャッジより遅く出る。`--release` を付けると最適化ビルドで測れる

## `match` の種類

実測結果。出力 `1 2  ⏎abc 0.3333333`（1 行目の末尾に空白 2 個）を、各種の期待出力と比べた。

| `match` | 判定 | 行末の空白 | 末尾の改行の有無 | 改行とスペースの違い |
| --- | --- | --- | --- | --- |
| `Exact` | バイト単位の完全一致 | WA | WA | WA |
| `Lines` | 行単位で比較 | **WA** | 無視 | WA |
| `SplitWhitespace` | 空白区切りのトークン列で比較 | 無視 | 無視 | 無視 |
| `Float` | 行ごとにトークンを比べ、数値は誤差つきで比較 | 無視 | 無視 | WA |
| `Checker` | 任意のコマンドで判定 | スクリプト次第 | スクリプト次第 | スクリプト次第 |

`Lines` は行末の空白で WA になる。`print!("{} ", x)` をループで出して末尾に空白が残る形は、AtCoder のジャッジでは AC になるのに `just test` では落ちる。迷ったら `SplitWhitespace` に変えて確かめる。

### `Float`

```yaml
match:
  Float:
    relative_error: 1e-7
    absolute_error: 1e-7   # ~ にするとその基準は使わない
```

- 問題文の「絶対誤差または相対誤差が 10^-7 以下」をそのまま書き写す
- 数値でないトークン（`Yes` など）は文字列として完全一致で比べる
- 行の区切りは見ているので、改行位置が違うと WA

### `Checker`

```yaml
match:
  Checker:
    cmd: python3 testcases/d-checker.py   # パッケージのディレクトリ基準
    shell: Bash
```

検証コマンドには、環境変数で各ファイルのパスが渡される。

| 環境変数 | 中身 |
| --- | --- |
| `INPUT` | 入力 |
| `EXPECTED_OUTPUT` | yml の `out:`（模範出力。答えが複数あるなら参考程度） |
| `ACTUAL_OUTPUT` | 自分のプログラムの出力 |

- 終了コード 0 なら AC、それ以外は WA
- stderr に出したものは `checker stderr:` として表示されるので、WA の理由を書いておく
- 実例: `abc/1xx/abc168/testcases/d-checker.py`（BFS の親を出す問題。出力された親が最短経路上にあるかを検証する）

検証スクリプトの雛形:

```python
import os
import sys

with open(os.environ["INPUT"]) as f:
    it = iter(f.read().split())
with open(os.environ["ACTUAL_OUTPUT"]) as f:
    out = f.read().split()

# ここで入力を読み、out が条件を満たすか検証する
ok = True
if not ok:
    print("WA の理由", file=sys.stderr)
    sys.exit(1)
```

## このリポジトリでの実例

| 種類 | ファイル |
| --- | --- |
| `match: SplitWhitespace` | `abc/4xx/abc477/testcases/b.yml` |
| `match: Float` | `abc/2xx/abc255/testcases/b.yml`, `other/dp/testcases/j.yml` |
| `match: Checker` | `abc/1xx/abc168/testcases/d.yml` |
| `type: Interactive` | `books/typical90/testcases/053.yml`（インタラクティブ問題。`cases` を持たない） |

`match: Lines` 以外の yml は `grep -rL 'match: Lines' --include='*.yml' abc books other` で一覧できる。
