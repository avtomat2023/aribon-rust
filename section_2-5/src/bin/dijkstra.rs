// 頂点数V, 辺数Eの無向グラフが与えられる。
// 頂点0から頂点(N-1)までの最短路のコストを、ダイクストラ法で計算せよ。
//
// 入力
// V E
// u_1 v_1 cost_1
// u_2 v_2 cost_2
// ...
// u_E v_E cost_E
//
// 出力
// 最短路のコスト

#[macro_use] extern crate atcoder_snippets;
use atcoder_snippets::read::*;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Clone)]
struct Edge {
    to: usize,
    cost: u32
}

#[derive(PartialEq, Eq)]
struct OpenNode {
    v: usize,
    dist: u32
}

impl Ord for OpenNode {
    fn cmp(&self, other: &OpenNode) -> Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for OpenNode {
    fn partial_cmp(&self, other: &OpenNode) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    read!(v_count = usize, _ = ());
    let mut edge_lists = vec![Vec::new(); v_count];
    readx_loop!(|v1 = usize, v2 = usize, cost = u32| {
        edge_lists[v1].push(Edge { to: v2, cost });
        edge_lists[v2].push(Edge { to: v1, cost });
    });

    let mut dists = vec![None; v_count];
    dists[0] = Some(0);
    let mut queue = BinaryHeap::new();
    queue.push(OpenNode { v: 0, dist: 0 });
    while let Some(nearest) = queue.pop() {
        if dists[nearest.v].unwrap_or(u32::max_value()) < nearest.dist {
            continue;
        }
        for e in &edge_lists[nearest.v] {
            let new_dist = nearest.dist + e.cost;
            if dists[e.to].unwrap_or(u32::max_value()) > new_dist {
                dists[e.to] = Some(new_dist);
                queue.push(OpenNode { v: e.to, dist: new_dist });
            }
        }
    }

    println!("{}", dists.last().unwrap().unwrap());
}
