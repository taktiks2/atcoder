// algo: data-structure, deque
use proconio::input;
use std::collections::VecDeque;

// クエリは 3 種: 1 x で先頭追加, 2 x で末尾追加, 3 x で前から x 番目を出力。
// 両端追加と先頭からのランダムアクセスの 3 つがすべて必要になる。
//
// VecDeque はリングバッファ実装なので push_front / push_back / インデックス参照 [i]
// がいずれも償却 O(1)。Q <= 2*10^5 のクエリを全体 O(Q) で捌けるので、そのまま素直に載る。
//
// std::collections::LinkedList でも両端追加は O(1) だが、3 番目のランダムアクセスが
// O(x) になり最悪 O(Q^2) で TLE。両端キューでインデックス参照もしたいときは
// VecDeque 一択と覚えておく。

fn main() {
    input! {
        q: usize,
        tx: [(usize, usize); q],
    };

    let mut queue = VecDeque::new();

    for (ti, xi) in tx {
        match ti {
            1 => queue.push_front(xi),
            2 => queue.push_back(xi),
            3 => println!("{}", queue[xi - 1]),
            _ => {}
        }
    }
}

// alt: 長さ 2Q+1 の固定バッファと (left, right) の 2 ポインタで手書きしても O(1) で回る。
//      中央 (= Q) を初期位置にして、push_front は左に伸ばし、push_back は右に伸ばす。
//      VecDeque のリングバッファをそのまま展開した形で、標準ライブラリが使えないときの
//      写経候補
// let cap = 2 * q + 1;
// let mut buf = vec![0usize; cap];
// let mut left = q;
// let mut right = q; // buf[left..right] が現在のキュー
// for (ti, xi) in tx {
//     match ti {
//         1 => { left -= 1; buf[left] = xi; }
//         2 => { buf[right] = xi; right += 1; }
//         3 => println!("{}", buf[left + xi - 1]),
//         _ => {}
//     }
// }

// alt: 2 本の Vec を「先頭側スタック」と「末尾側 Vec」として持つ実装。push_front は
//      front.push、push_back は back.push、参照は x が front の長さ以下なら front を
//      逆順に、そうでなければ back を先頭から引く。全操作 O(1) で、リングバッファを
//      使わず両端キューを組み立てる典型
// let mut front: Vec<usize> = Vec::new(); // front.last() が現在の先頭
// let mut back: Vec<usize> = Vec::new();  // back.last() が現在の末尾
// for (ti, xi) in tx {
//     match ti {
//         1 => front.push(xi),
//         2 => back.push(xi),
//         3 => {
//             let idx = xi - 1;
//             let v = if idx < front.len() {
//                 front[front.len() - 1 - idx]
//             } else {
//                 back[idx - front.len()]
//             };
//             println!("{v}");
//         }
//         _ => {}
//     }
// }
