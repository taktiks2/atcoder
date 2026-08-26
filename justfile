# AtCoder リポジトリのタスクランナー (cargo-compete のラッパー)
#
# test/submit などは各コンテストのパッケージディレクトリ内で実行する必要があるため、
# コンテスト名からディレクトリを解決して cd してから実行する。

set shell := ["bash", "-uc"]

# レシピ一覧を表示
default:
    @just --list --unsorted

# 2025-03 以降 Cloudflare により `cargo compete login` は使えないため手動コピー方式。
# 提出も終了済みコンテストへは CLI 不可 = ブラウザのみ。開催中コンテストは just submit 可。
# AtCoder に再ログイン: ブラウザでログイン後、REVEL_SESSION Cookie を貼り付けて保存
login:
    #!/usr/bin/env bash
    set -euo pipefail
    jar="$HOME/Library/Application Support/cargo-compete/cookies.jsonl"
    open "https://atcoder.jp/login"
    echo "1. 開いたページでログインする"
    echo "2. 開発者ツール (Cmd+Opt+I) → Application → Cookies → https://atcoder.jp"
    echo "3. REVEL_SESSION の「値」をコピーして貼り付け、Enter"
    read -rp "REVEL_SESSION: " session
    session="${session#REVEL_SESSION=}"; session="${session//[$'\t\r\n \"']/}"
    if [[ "$session" != *UserName%3A* ]]; then
        echo "エラー: 未ログインの匿名セッションです。ブラウザでログインしてからやり直してください" >&2
        exit 1
    fi
    ts="$(grep -oE '_TS%3A[0-9]+' <<<"$session" | sed 's/_TS%3A//' || true)"
    if [[ -n "$ts" ]]; then
        exp="$(date -u -d "@$ts" +%Y-%m-%dT%H:%M:%SZ 2>/dev/null || date -u -r "$ts" +%Y-%m-%dT%H:%M:%SZ)"
    else
        exp="$(date -u -d '+180 days' +%Y-%m-%dT%H:%M:%SZ 2>/dev/null || date -u -v+180d +%Y-%m-%dT%H:%M:%SZ)"
    fi
    mkdir -p "$(dirname "$jar")"
    [[ -f "$jar" ]] && cp "$jar" "$jar.bak"
    jq -nc --arg rc "REVEL_SESSION=$session" --arg exp "$exp" \
        '{raw_cookie: $rc, path: ["/", true], domain: {HostOnly: "atcoder.jp"}, expires: {AtUtc: $exp}}' > "$jar"
    user="$(grep -oE 'UserScreenName%3A[A-Za-z0-9_]+' <<<"$session" | sed 's/UserScreenName%3A//')"
    echo "OK: ${user:-(不明)} としてセッションを保存しました (期限 $exp)"

# ログイン状態を確認 (Cookie のユーザー名 + 要ログイン API で E2E 確認)
whoami:
    @grep -o 'UserScreenName%3A[A-Za-z0-9_]*' "$HOME/Library/Application Support/cargo-compete/cookies.jsonl" 2>/dev/null | sed 's/UserScreenName%3A/Cookie: /' || { echo "未ログイン (匿名セッション)"; exit 1; }
    @d=$(ls -d abc/* 2>/dev/null | head -1) && cd "$d" && cargo compete retrieve submission-summaries > /dev/null && echo "API 疎通: OK (ログイン有効)"

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

# 終了済みコンテストは CLI 提出不可のため、ブラウザ提出を最短化する。
# URL は Cargo.toml のメタデータから取得 (tessoku-book 等、URL が規則的でない場合に対応)。
# ソースをクリップボードにコピーして問題ページを開く: just clip abc472 a
clip contest problem:
    d="$("{{ just_executable() }}" _dir {{ contest }})" && \
    url=$(grep -E 'alias = "{{ problem }}"|^{{ problem }} = ' "$d/Cargo.toml" | grep -oE 'https://[^"]+' | head -1) && \
    pbcopy < "$d/src/bin/{{ problem }}.rs" && \
    echo "クリップボードにコピー: $d/src/bin/{{ problem }}.rs" && \
    open "${url:-https://atcoder.jp/contests/{{ contest }}/tasks/{{ contest }}_{{ problem }}}"

# 問題ページとソース/テストを開く: just open abc338
open contest:
    cd "$("{{ just_executable() }}" _dir {{ contest }})" && cargo compete open

# テストケース再取得: just retrieve abc338 (--full でシステムテスト)
retrieve contest *flags:
    cd "$("{{ just_executable() }}" _dir {{ contest }})" && cargo compete retrieve testcases {{ flags }}

# タグで過去問を検索 (部分一致・大文字小文字無視): just algo bfs
# タグの語彙は TAGS.md 参照。初出のタグは TAGS.md に追記してから使う
algo tag:
    @grep -rni --include='*.rs' '^// algo:.*{{ tag }}' $(ls -d abc arc agc ahc books other 2>/dev/null) || echo "該当なし: {{ tag }}"

# 全タグの使用回数一覧: just algos
algos:
    @grep -rh --include='*.rs' '^// algo:' $(ls -d abc arc agc ahc books other 2>/dev/null) \
        | sed 's|^// algo: *||' | tr ',' '\n' | sed 's/^ *//; s/ *$//' | sort | uniq -c | sort -rn

alias n := new
alias t := test
alias s := submit
alias o := open
alias c := clip
alias a := algo

# コンテスト名からディレクトリを解決 (内部用)
_dir contest:
    @d=$(ls -d abc/{{ contest }} arc/{{ contest }} agc/{{ contest }} ahc/{{ contest }} books/{{ contest }} other/{{ contest }} 2>/dev/null | head -1); \
    if [ -n "$d" ]; then echo "$d"; else echo "コンテストが見つかりません: {{ contest }}" >&2; exit 1; fi
