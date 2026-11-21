// algo:
// solve:
#![allow(unused_imports)]
use itertools::Itertools;
use pathfinding::directed::bfs;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        uvc: [(Usize1, Usize1, usize); m],
    };

    let mut graph: Vec<Vec<(usize, usize)>> = vec![vec![]; n];

    for (u, v, c) in uvc {
        graph[u].push((v, c));
    }

    let mut arrival_times = vec![-1i64; n];
    // (行き先, 直前の色, 通る回数)
    let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::new();
    queue.push_back((0, 0, 0));

    let mut ans = -1;
    while let Some((target, last_color, times)) = queue.pop_front() {
        if target == n - 1 {
            ans = times as i64;
            break;
        }
        for &(to, color) in &graph[target] {
            if arrival_times[to] < 2 && last_color != color {
                queue.push_back((to, color, times + 1));
                arrival_times[to] += 1;
            }
        }
    }

    println!("{ans}");
}
