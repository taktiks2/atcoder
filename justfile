# AtCoder リポジトリのタスクランナー (cargo-compete のラッパー)
#
# test/submit などは各コンテストのパッケージディレクトリ内で実行する必要があるため、
# コンテスト名からディレクトリを解決して cd してから実行する。

set shell := ["bash", "-uc"]

# レシピ一覧を表示
default:
    @just --list --unsorted

# AtCoder にログイン
login:
    cargo compete login atcoder

# 新規コンテスト作成: just new abc338 (生成先は compete.toml が種別ごとに振り分け)
new contest:
    cargo compete new {{ contest }}

# サンプルでテスト: just test abc338 a (追加フラグ可: --release など)
# mkdir: 旧版 cargo-compete で作ったパッケージには yml が参照する testcases/<問題>/ が無く、
# テストスイートの解決に失敗するため作っておく (新規パッケージでは生成済みで無害)
test contest problem *flags:
    d="$("{{ just_executable() }}" _dir {{ contest }})" && mkdir -p "$d/testcases/{{ problem }}" && cd "$d" && cargo compete test {{ problem }} {{ flags }}

# 提出 (テスト通過時のみ): just submit abc338 a (--no-test / --no-watch 可)
submit contest problem *flags:
    d="$("{{ just_executable() }}" _dir {{ contest }})" && mkdir -p "$d/testcases/{{ problem }}" && cd "$d" && cargo compete submit {{ problem }} {{ flags }}

# 問題ページとソース/テストを開く: just open abc338
open contest:
    cd "$("{{ just_executable() }}" _dir {{ contest }})" && cargo compete open

# テストケース再取得: just retrieve abc338 (--full でシステムテスト)
retrieve contest *flags:
    cd "$("{{ just_executable() }}" _dir {{ contest }})" && cargo compete retrieve testcases {{ flags }}

alias n := new
alias t := test
alias s := submit
alias o := open

# コンテスト名からディレクトリを解決 (内部用)
_dir contest:
    @d=$(ls -d abc/{{ contest }} arc/{{ contest }} agc/{{ contest }} ahc/{{ contest }} books/{{ contest }} other/{{ contest }} 2>/dev/null | head -1); \
    if [ -n "$d" ]; then echo "$d"; else echo "コンテストが見つかりません: {{ contest }}" >&2; exit 1; fi
